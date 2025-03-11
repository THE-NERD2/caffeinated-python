cdef extern from "jni_helpers.h":
    unsigned char* create_JVM(const char*)
    unsigned char* instantiate_class(unsigned char*, const char*)