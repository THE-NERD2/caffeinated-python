package org.caffeinatedpython.interop;

public class PyInterop {
    static {
        System.loadLibrary("caffeinatedpython");
        init();
    }
    private static native void init();
    public static native int createPythonScope();
    public static native void closePythonScope(int scopeIndex);
    public static native int performOperation(int scopeIndex, String operation);
    public static native Object operateAndExtract(int scopeIndex, String operation, String type);
}