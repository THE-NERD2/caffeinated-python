package org.caffeinatedpython.interop;

import org.caffeinatedpython.PyModule;

import java.util.HashMap;

public class PyInterop {
    static {
        System.loadLibrary("caffeinatedpython");
    }
    public static native HashMap<String, PyModule> importPythonModule(String name);
}