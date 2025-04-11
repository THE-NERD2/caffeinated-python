package org.caffeinatedpython.exceptions

import org.caffeinatedpython.interop.PyInterop

class PythonException(private val errorIndex: Int): Exception("An error occurred in PythonScope") {
    fun printPythonStackTrace() = PyInterop.printPythonStackTrace(errorIndex)
}