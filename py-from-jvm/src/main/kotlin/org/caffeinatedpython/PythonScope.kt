package org.caffeinatedpython

import kotlinx.coroutines.CoroutineScope
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.coroutineScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.withContext
import kotlinx.coroutines.yield
import org.caffeinatedpython.interop.PyInterop
import kotlin.reflect.KClass

class PythonScope private constructor(scope: CoroutineScope): CoroutineScope by scope {
    companion object {
        suspend fun pythonScope(block: suspend PythonScope.() -> Unit) = coroutineScope {
            PyInterop.createPythonScope()
            val pythonScope = PythonScope(this)
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
        private var calculated = false

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
            launch {
                yield()
                now()
            }
        }

        constructor(identifier: String) : this(identifierName = identifier)

        operator fun get(name: String) = PyAny(subject = this, identifierName = name)
        fun <T : Any> extract(clazz: KClass<T>) =
            PyInterop.operateAndExtract(operation.toString(), clazz.simpleName!!) as T

        fun now(): PyAny {
            if(!calculated) {
                operation = Operation.EXISTING_VAR(PyInterop.performOperation(operation.toString()))
                calculated = true
            }
            return this
        }
    }

    fun import(name: String) = PyAny(moduleName = name)
}