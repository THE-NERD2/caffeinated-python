package org.caffeinatedpython.exceptions

class NonexistentMemberAccessException(member: String): Exception("Member $member does not exist")