use std::ffi::CStr;
use std::os::raw::c_char;
use jni::{sys::_jobject, InitArgsBuilder, JavaVM};

type JavaVM_ = jni::sys::JavaVM;

#[no_mangle]
pub unsafe extern "system" fn create_JVM(classpath_raw: *const c_char) -> *mut JavaVM_ {
    let classpath = CStr::from_ptr(classpath_raw).to_str().unwrap();
    let jvm_args = InitArgsBuilder::new().option(String::from("-Djava.class.path=") + classpath).build().unwrap();
    return JavaVM::new(jvm_args).unwrap().get_java_vm_pointer();
}

// TODO: allow constructors with arguments
#[no_mangle]
pub unsafe extern "system" fn instantiate_class(jvm_raw: *mut JavaVM_, classname_raw: *const c_char) -> *mut _jobject {
    let jvm = JavaVM::from_raw(jvm_raw).unwrap();
    let classname = CStr::from_ptr(classname_raw).to_str().unwrap();
    let mut env = jvm.attach_current_thread().unwrap();
    let class = env.find_class(classname).unwrap();
    return env.new_object(class, "()V", &[]).unwrap().as_raw();
}