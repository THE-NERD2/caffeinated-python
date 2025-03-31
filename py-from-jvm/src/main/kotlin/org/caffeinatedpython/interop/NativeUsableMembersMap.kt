package org.caffeinatedpython.interop

import org.caffeinatedpython.PyMember

class NativeUsableMembersMap {
    internal val members = HashMap<String, PyMember>()
    fun put(key: String, value: PyMember) { members.put(key, value) }
}