package org.caffeinatedpython

import org.caffeinatedpython.exceptions.PythonException
import org.junit.jupiter.api.Test

class Tests {
    @Test
    fun getVersion() {
        Python.pythonScope {
            println(import("numpy")["__version__"].extract(String::class))
        }
    }
    @Test
    fun getIndex() {
        Python.pythonScope {
            val version = import("numpy")["__version__"].now()
            println(version.extract(String::class))
        }
    }
    @Test
    fun getMaxInt() {
        Python.pythonScope {
            println(import("sys")["maxsize"].extract(Long::class))
        }
    }
    @Test
    fun exception() {
        Python.pythonScope {
            try {
                PyAny("absolutely_does_not_exist").now()
            } catch(e: PythonException) {
                e.printPythonStackTrace()
            }
        }
    }
    @Test
    fun extractionException() {
        Python.pythonScope {
            try {
                import("sys")["maxsize"].extract(String::class)
            } catch(e: Exception) {
                e.printStackTrace()
            }
        }
    }
}