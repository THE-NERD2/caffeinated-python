use jni::{objects::{JClass, JObject}, JNIEnv};

#[no_mangle]
pub extern "system" fn Java_org_caffeinatedpython_interop_PyInterop_importPythonModule<'a>(mut env: JNIEnv<'a>, _: JClass<'a>, name: JObject<'a>) -> JObject<'a> {
    let obj = env.new_object("Ljava/util/HashMap;", "()V", &[]).unwrap();
    obj
}