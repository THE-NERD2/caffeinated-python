package org.caffeinatedpython

import org.junit.jupiter.api.Test

class Tests {
    @Test
    fun test() {
        PyModule("numpy")["__version__"]
    }
    @Test
    fun shouldFail() {
        PyModule("numpy")["absolutely_does_not_exist"]
    }
}