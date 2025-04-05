package org.caffeinatedpython

internal enum class Operation {
    IMPORT,
    MEMBER_ACCESS,
    TOP_LEVEL,
    EXISTING_VAR;

    var subject: Operation? = null; private set
    var predicate: String? = null; private set
    override fun toString() = when(this) {
        IMPORT -> "*IMPORT $predicate"
        MEMBER_ACCESS -> "$subject $predicate"
        TOP_LEVEL -> "*BUILTIN $predicate"
        EXISTING_VAR -> "*EXISTING $predicate"
    }
    operator fun invoke(index: Int): Operation {
        predicate = "$index"
        return this
    }
    operator fun invoke(name: String): Operation {
        predicate = name
        return this
    }
    operator fun invoke(previous: Operation, name: String): Operation {
        subject = previous
        predicate = name
        return this
    }
}