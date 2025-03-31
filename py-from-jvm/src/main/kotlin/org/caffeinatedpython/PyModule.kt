package org.caffeinatedpython

import org.caffeinatedpython.exceptions.NonexistentMemberAccessException
import org.caffeinatedpython.interop.PyInterop

class PyModule(val name: String) {
    private val members = PyInterop.importPythonModule(name).members
    override fun toString() = name
    operator fun get(name: String) = members[name] ?: throw NonexistentMemberAccessException(name)
}