use std::{ffi::CString, ops::Deref};
use libc::{RTLD_LAZY, RTLD_GLOBAL};

use jni::{objects::{JClass, JObject, JString, JValueGen}, JNIEnv};

use pyo3::{prelude::*, types::{PyList, PyTuple}};

#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_init(_: JNIEnv<'_>, _: JClass<'_>) {
    // Manually include python symbols
    unsafe {
        let libpython_str = CString::new("libpython3.11.so").unwrap();
        let handle = libc::dlopen(libpython_str.as_ptr(), RTLD_LAZY | RTLD_GLOBAL);
        if handle.is_null() {
            eprintln!("Failed to import python");
        }
    }

    pyo3::prepare_freethreaded_python();
}

#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_importPythonModule<'a>(mut env: JNIEnv<'a>, _: JClass<'a>, name: JObject<'a>) -> JObject<'a> {
    Python::with_gil(|py| {
        let obj = Box::new(env.new_object("Lorg/caffeinatedpython/interop/NativeUsableMembersMap;", "()V", &[]).unwrap());
        let module_name: String = env.get_string(&JString::from(name)).unwrap().into();
        let builtins = PyModule::import(py, "builtins").unwrap();
        let module = PyModule::import(py, &module_name[..]).unwrap();
        let keys: Bound<'_, PyList> = builtins
            .getattr("dir").unwrap()
            .call1(PyTuple::new(py, &[module]).unwrap()).unwrap()
            .downcast_into().unwrap();
        for key in keys.iter() {
            let key_name = key.extract::<&str>().unwrap();
            let key_name_jstr = env.new_string(key_name).unwrap();
            let pymember = env.new_object("Lorg/caffeinatedpython/PyMember;", "()V", &[]).unwrap();
            let _ = env.call_method(obj.deref(), "put", "(Ljava/lang/String;Lorg/caffeinatedpython/PyMember;)V", &[
                JValueGen::Object(&key_name_jstr),
                JValueGen::Object(&pymember)
            ]); // Assignment to avoid unwrapping for no reason
        }
        *obj
    })
}