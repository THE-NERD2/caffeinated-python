package org.caffeinatedpython

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import org.caffeinatedpython.interop.PyInterop
import kotlin.coroutines.CoroutineContext
import kotlin.reflect.KClass

class PythonScope private constructor() : CoroutineScope {
    companion object {
        @Synchronized
        fun pythonScope(block: PythonScope.() -> Unit) {
            PyInterop.createPythonScope()
            val pythonScope = PythonScope()
            pythonScope.block()
            PyInterop.closePythonScope()
        }
    }

    inner class PyAny internal constructor(
        subject: PyAny? = null,
        moduleName: String? = null,
        identifierName: String? = null,
        objectIndex: Int? = null
    ) {
        private var operation: Operation

        init {
            if (moduleName != null) {
                operation = Operation.IMPORT(moduleName)
            } else if (identifierName != null) {
                if (subject != null) {
                    operation = Operation.MEMBER_ACCESS(subject.operation, identifierName)
                } else {
                    operation = Operation.TOP_LEVEL(identifierName)
                }
            } else {
                operation = Operation.EXISTING_VAR(objectIndex!!)
            }
        }

        constructor(identifier: String) : this(identifierName = identifier)

        operator fun get(name: String) = PyAny(subject = this, identifierName = name)
        fun <T : Any> extract(clazz: KClass<T>) =
            PyInterop.operateAndExtract(operation.toString(), clazz.simpleName!!) as T

        fun now(): PyAny {
            operation = Operation.EXISTING_VAR(PyInterop.performOperation(operation.toString()))
            return this
        }
    }

    override val coroutineContext: CoroutineContext
        get() = Dispatchers.Default

    fun import(name: String) = PyAny(moduleName = name)
}