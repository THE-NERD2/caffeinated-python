from caffeinated_python_rust cimport *

cdef unsigned char* jvm

def createJVM(str classpath):
    global jvm
    jvm = create_JVM(classpath.encode("UTF-8"))

def instantiateClass(str classname):
    modified_classname = classname.replace(".", "/")
    return instantiate_class(jvm, modified_classname.encode("UTF-8"))