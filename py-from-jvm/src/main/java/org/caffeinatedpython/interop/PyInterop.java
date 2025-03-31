package org.caffeinatedpython.interop;

public class PyInterop {
    static {
        System.loadLibrary("caffeinatedpython");
        init();
    }
    private static native void init();
    public static native NativeUsableMembersMap importPythonModule(String name);
}