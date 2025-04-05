package org.caffeinatedpython.exceptions

import org.caffeinatedpython.interop.PyInterop

class PythonException(private val errorIndex: Int): Exception("An error occurred in Python") {
    fun printPythonStackTrace() = PyInterop.printPythonStackTrace(errorIndex)
}