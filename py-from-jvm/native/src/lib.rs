// TODO: error checking

use std::{ffi::CString, sync::{mpsc::{self, Receiver, Sender}, Mutex}, thread::{self, JoinHandle}};
use libc::{RTLD_LAZY, RTLD_GLOBAL};

use jni::{objects::{JClass, JObject, JString, JValueGen}, JNIEnv};

use pyo3::{prelude::*, types::PyInt};

#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_init(_: JNIEnv<'_>, _: JClass<'_>) {
    // Manually include python symbols
    unsafe {
        // TODO: support more python versions
        let libpython_str = CString::new("libpython3.11.so").unwrap();
        let handle = libc::dlopen(libpython_str.as_ptr(), RTLD_LAZY | RTLD_GLOBAL);
        if handle.is_null() {
            panic!("Failed to import python");
        }
    }

    pyo3::prepare_freethreaded_python();
}

#[allow(dead_code)]
struct PythonScope {
    handle: JoinHandle<()>,
    tx: Sender<String>,
    rx: Receiver<Response>
}
static PYTHON_SCOPES: Mutex<Vec<PythonScope>> = Mutex::new(Vec::new());

enum Response {
    Index(i32),
    String(String),
    Int(i64),
    Float(f64)
}

#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_createPythonScope(_: JNIEnv<'_>, _: JClass<'_>) -> i32 {
    let (tx, rx_thread) = mpsc::channel();
    let (tx_thread, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        Python::with_gil(|py| {
            let mut msg: String;
            let mut vars: Vec<Bound<'_, PyAny>> = Vec::new();
            loop {
                match rx_thread.recv() {
                    Ok(v) => { msg = v }
                    Err(_) => break
                }
                match msg {
                    val if val == "*EXIT".to_string() => break,
                    op => {
                        let mut tokens = op.split(" ");
                        let mut current_obj: Bound<'_, PyAny> = PyInt::new(py, 0).as_any().clone(); // Placeholder
                        loop {
                            let next_token = match tokens.next() {
                                Some(tok) => tok,
                                None => {
                                    let res = vars.len();
                                    vars.push(current_obj);
                                    let _ = tx_thread.send(Response::Index(res.try_into().unwrap()));
                                    break;
                                }
                            };
                            match next_token {
                                "*IMPORT" => {
                                    current_obj = PyModule::import(py, tokens.next().unwrap()).unwrap().as_any().clone();
                                }
                                "*BUILTIN" => {
                                    current_obj = PyModule::import(py, "builtins").unwrap().getattr(tokens.next().unwrap()).unwrap();
                                }
                                "*EXISTING" => {
                                    current_obj = vars[tokens.next().unwrap().parse::<i32>().unwrap() as usize].clone();
                                }
                                "*EXTRACT" => {
                                    match tokens.next().unwrap() {
                                        "String" => {
                                            let val: &str = current_obj.extract().unwrap();
                                            let _ = tx_thread.send(Response::String(val.to_string()));
                                        }
                                        "Byte" | "Short" | "Int" | "Long" => {
                                            let val: i64 = current_obj.extract().unwrap();
                                            let _ = tx_thread.send(Response::Int(val));
                                        }
                                        "Float" | "Double" => {
                                            let val: f64 = current_obj.extract().unwrap();
                                            let _ = tx_thread.send(Response::Float(val));
                                        }
                                        _ => {
                                            // Respond with a command to throw an exception for illegal extraction
                                        }
                                    }
                                    break;
                                }
                                tok => {
                                    current_obj = current_obj.getattr(tok).unwrap();
                                }
                            }
                        }
                    }
                }
            }
        });
    });
    let mut python_scopes = PYTHON_SCOPES.lock().unwrap();
    let ret = python_scopes.len();
    python_scopes.push(PythonScope {
        handle, tx, rx
    });
    return ret.try_into().unwrap();
}
#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_closePythonScope(_: JNIEnv<'_>, _: JClass<'_>, index: i32) {
    let scope = &PYTHON_SCOPES.lock().unwrap()[index as usize];
    let _ = scope.tx.send("*EXIT".to_string());
}
#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_performOperation(mut env: JNIEnv<'_>, _: JClass<'_>, index: i32, operation: JObject<'_>) -> i32 {
    let scope = &PYTHON_SCOPES.lock().unwrap()[index as usize];
    let operation_rstr: String = env.get_string(&JString::from(operation)).unwrap().into();
    let _ = scope.tx.send(operation_rstr);
    match scope.rx.recv().unwrap() {
        Response::Index(ret) => return ret,
        _ => panic!("Did not get an index from a non-extraction operation")
    }
}
#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_operateAndExtract<'a>(mut env: JNIEnv<'a>, _: JClass<'a>, index: i32, operation: JObject<'a>, extract_type: JObject<'a>) -> JObject<'a> {
    let scope = &PYTHON_SCOPES.lock().unwrap()[index as usize];
    let operation_rstr: String = env.get_string(&JString::from(operation)).unwrap().into();
    let extract_type_rstr: String = env.get_string(&JString::from(extract_type)).unwrap().into();
    let _ = scope.tx.send([operation_rstr, "*EXTRACT".to_string(), extract_type_rstr].join(" "));
    match scope.rx.recv().unwrap() {
        Response::String(res) => return env.new_string(res).unwrap().into(),
        Response::Int(res) => return env.new_object("Ljava/lang/Long;", "(J)V", &[JValueGen::Long(res)]).unwrap(),
        Response::Float(res) => return env.new_object("Ljava/lang/Double;", "(D)V", &[JValueGen::Double(res)]).unwrap(),
        _ => panic!("Did not get a valid response from an extraction")
    }
}