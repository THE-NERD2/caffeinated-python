package org.caffeinatedpython

import kotlinx.coroutines.runBlocking
import org.caffeinatedpython.exceptions.PythonException
import org.junit.jupiter.api.Test

class Tests {
    @Test
    fun getVersion() = runBlocking {
        PythonScope.pythonScope {
            println(import("numpy")["__version__"].extract(String::class))
        }
    }
    @Test
    fun getIndex() = runBlocking {
        PythonScope.pythonScope {
            val version = import("numpy")["__version__"].now()
            println(version.extract(String::class))
        }
    }
    @Test
    fun getMaxInt() = runBlocking {
        PythonScope.pythonScope {
            println(import("sys")["maxsize"].extract(Long::class))
        }
    }
    @Test
    fun exception() = runBlocking {
        PythonScope.pythonScope {
            try {
                PyAny("absolutely_does_not_exist").now()
            } catch(e: PythonException) {
                e.printPythonStackTrace()
            }
        }
    }
    @Test
    fun extractionException() = runBlocking {
        PythonScope.pythonScope {
            try {
                import("sys")["maxsize"].extract(String::class)
            } catch(e: Exception) {
                e.printStackTrace()
            }
        }
    }
}