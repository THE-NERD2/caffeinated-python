package org.caffeinatedpython

import org.caffeinatedpython.interop.PyInterop
import kotlin.reflect.KClass

class Python private constructor(private val threadIndex: Int) {
    companion object {
        fun pythonScope(block: Python.() -> Unit) {
            val threadIndex = PyInterop.createPythonScope()
            val python = Python(threadIndex)
            python.block()
            PyInterop.closePythonScope(threadIndex)
        }
    }

    inner class PyAny internal constructor(
        subject: PyAny? = null,
        module_name: String? = null,
        identifier_name: String? = null,
        object_index: Int? = null
    ) {
        private var operation: Operation

        init {
            if(module_name != null) {
                operation = Operation.IMPORT(module_name)
            } else if(identifier_name != null) {
                if(subject != null) {
                    operation = Operation.MEMBER_ACCESS(subject.operation, identifier_name)
                } else {
                    operation = Operation.TOP_LEVEL(identifier_name)
                }
            } else {
                operation = Operation.EXISTING_VAR(object_index!!)
            }
        }
        constructor(identifier: String): this(identifier_name = identifier)
        operator fun get(name: String) = PyAny(subject = this, identifier_name = name)
        fun <T: Any> extract(clazz: KClass<T>) = PyInterop.operateAndExtract(threadIndex, operation.toString(), clazz.simpleName!!) as T
        fun now(): PyAny {
            operation = Operation.EXISTING_VAR(PyInterop.performOperation(threadIndex, operation.toString()))
            return this
        }
    }
    fun import(name: String) = PyAny(module_name = name)
}