package org.caffeinatedpython.interop;

import org.caffeinatedpython.PyMember;

import java.util.HashMap;

public class PyInterop {
    static {
        System.loadLibrary("caffeinatedpython");
    }
    public static native HashMap<String, PyMember> importPythonModule(String name);
}