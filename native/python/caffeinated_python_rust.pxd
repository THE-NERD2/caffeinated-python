cdef extern from "caffeinated_python_rust.h":
    unsigned char* create_JVM(const char*)
    unsigned char* instantiate_class(unsigned char*, const char*)