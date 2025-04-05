package org.caffeinatedpython

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
}