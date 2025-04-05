use std::{ffi::CString, sync::{mpsc::{self, Receiver, Sender}, Mutex}, thread::{self, JoinHandle}};
use libc::{RTLD_LAZY, RTLD_GLOBAL};

use jni::{objects::{JClass, JObject, JString, JThrowable, JValueGen}, JNIEnv};

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
}

#[allow(dead_code)]
struct PythonScope {
    handle: JoinHandle<()>,
    tx: Sender<String>,
    rx: Receiver<Response>
}
static PYTHON_SCOPE: Mutex<Option<PythonScope>> = Mutex::new(None);

enum Exception {
    PythonException(i32),
    IllegalExtraction,
    WrongTypeExtraction
}
enum Response {
    Exception(Exception),
    Index(i32),
    String(String),
    Int(i64),
    Float(f64)
}

#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_createPythonScope(_: JNIEnv<'_>, _: JClass<'_>) {
    let (tx, rx_thread) = mpsc::channel();
    let (tx_thread, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        Python::with_gil(|py| {
            let mut msg: String;
            let mut vars: Vec<Bound<'_, PyAny>> = Vec::new();
            let mut exceptions: Vec<PyErr> = Vec::new();
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
                                    current_obj = match PyModule::import(py, tokens.next().unwrap()) {
                                        Ok(module) => module.as_any().clone(),
                                        Err(error) => {
                                            let res = exceptions.len();
                                            exceptions.push(error);
                                            let _ = tx_thread.send(Response::Exception(Exception::PythonException(res.try_into().unwrap())));
                                            break;
                                        }
                                    }
                                }
                                "*BUILTIN" => {
                                    current_obj = match PyModule::import(py, "builtins").unwrap().getattr(tokens.next().unwrap()) {
                                        Ok(val) => val,
                                        Err(error) => {
                                            let res = exceptions.len();
                                            exceptions.push(error);
                                            let _ = tx_thread.send(Response::Exception(Exception::PythonException(res.try_into().unwrap())));
                                            break;
                                        }
                                    }
                                }
                                "*EXISTING" => {
                                    current_obj = vars[tokens.next().unwrap().parse::<i32>().unwrap() as usize].clone();
                                }
                                "*EXTRACT" => {
                                    match tokens.next().unwrap() {
                                        "String" => {
                                            let val: &str = match current_obj.extract() {
                                                Ok(val) => val,
                                                Err(_) => {
                                                    let _ = tx_thread.send(Response::Exception(Exception::WrongTypeExtraction));
                                                    break;
                                                }
                                            };
                                            let _ = tx_thread.send(Response::String(val.to_string()));
                                        }
                                        "Byte" | "Short" | "Int" | "Long" => {
                                            let val: i64 = match current_obj.extract() {
                                                Ok(val) => val,
                                                Err(_) => {
                                                    let _ = tx_thread.send(Response::Exception(Exception::WrongTypeExtraction));
                                                    break;
                                                }
                                            };
                                            let _ = tx_thread.send(Response::Int(val));
                                        }
                                        "Float" | "Double" => {
                                            let val: f64 = match current_obj.extract() {
                                                Ok(val) => val,
                                                Err(_) => {
                                                    let _ = tx_thread.send(Response::Exception(Exception::WrongTypeExtraction));
                                                    break;
                                                }
                                            };
                                            let _ = tx_thread.send(Response::Float(val));
                                        }
                                        _ => {
                                            let _ = tx_thread.send(Response::Exception(Exception::IllegalExtraction));
                                        }
                                    }
                                    break;
                                }
                                "*THROW" => {
                                    let index = tokens.next().unwrap().parse::<i32>().unwrap();
                                    exceptions[index as usize].display(py);
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
    *PYTHON_SCOPE.lock().unwrap() = Some(PythonScope {
        handle, tx, rx
    })
}
#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_closePythonScope(_: JNIEnv<'_>, _: JClass<'_>) {
    let _ = PYTHON_SCOPE.lock().unwrap().as_ref().unwrap().tx.send("*EXIT".to_string());
}
#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_performOperation(mut env: JNIEnv<'_>, _: JClass<'_>, operation: JObject<'_>) -> i32 {
    let scope_raw = PYTHON_SCOPE.lock().unwrap();
    let scope = scope_raw.as_ref().unwrap();
    let operation_rstr: String = env.get_string(&JString::from(operation)).unwrap().into();
    let _ = scope.tx.send(operation_rstr);
    match scope.rx.recv().unwrap() {
        Response::Index(ret) => return ret,
        Response::Exception(err) => match err {
            Exception::PythonException(index) => {
                let exception = env.new_object("Lorg/caffeinatedpython/exceptions/PythonException;", "(I)V", &[
                    JValueGen::Int(index)
                ]).unwrap();
                let _ = env.throw(JThrowable::from(exception));
            }
            _ => panic!("Got an extraction-related exception from a non-extraction operation")
        }
        _ => panic!("Did not get an index from a non-extraction operation")
    }
    // Return error value
    -1
}
#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_operateAndExtract<'a>(mut env: JNIEnv<'a>, _: JClass<'a>, operation: JObject<'a>, extract_type: JObject<'a>) -> JObject<'a> {
    let scope_raw = PYTHON_SCOPE.lock().unwrap();
    let scope = scope_raw.as_ref().unwrap();
    let operation_rstr: String = env.get_string(&JString::from(operation)).unwrap().into();
    let extract_type_rstr: String = env.get_string(&JString::from(extract_type)).unwrap().into();
    let _ = scope.tx.send([operation_rstr, "*EXTRACT".to_string(), extract_type_rstr.clone()].join(" "));
    match scope.rx.recv().unwrap() {
        Response::String(res) => return env.new_string(res).unwrap().into(),
        Response::Int(res) => return env.new_object("Ljava/lang/Long;", "(J)V", &[JValueGen::Long(res)]).unwrap(),
        Response::Float(res) => return env.new_object("Ljava/lang/Double;", "(D)V", &[JValueGen::Double(res)]).unwrap(),
        Response::Exception(err) => {
            // Constructing error value before error thrown
            let ret = env.new_object("Ljava/lang/Object;", "()V", &[]).unwrap();
            match err {
                Exception::IllegalExtraction => {
                    let _ = env.throw_new("Ljava/lang/UnsupportedOperationException;", ["Cannot extract to ", extract_type_rstr.as_str()].join(""));
                }
                Exception::WrongTypeExtraction => {
                    let _ = env.throw_new("Ljava/lang/UnsupportedOperationException;", "Extracted to wrong type");
                }
                Exception::PythonException(index) => {
                    let exception = env.new_object("Lorg/caffeinatedpython/exceptions/PythonException;", "(I)V", &[
                        JValueGen::Int(index)
                    ]).unwrap();
                    let _ = env.throw(JThrowable::from(exception));
                }
            }
            ret
        }
        _ => panic!("Got an invalid response from an extraction operation")
    }
}
#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_printPythonStackTrace(_: JNIEnv<'_>, _: JClass<'_>, index: i32) {
    let scope_raw = PYTHON_SCOPE.lock().unwrap();
    let scope = scope_raw.as_ref().unwrap();
    let _ = scope.tx.send(format!("*THROW {}", index));
}