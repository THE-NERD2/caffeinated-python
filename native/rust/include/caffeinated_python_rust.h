#ifndef RUST_H
#define RUST_H

#ifdef __cplusplus
#define EXPORT extern "C"
#else
#define EXPORT
#endif

EXPORT unsigned char* create_JVM(const char*); // JavaVM* (classpath)
EXPORT unsigned char* instantiate_class(unsigned char*, const char*); // JObject* (jvm, classname)

#endif