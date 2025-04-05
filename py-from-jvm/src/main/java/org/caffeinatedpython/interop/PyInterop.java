package org.caffeinatedpython.interop;

public class PyInterop {
    static {
        System.loadLibrary("caffeinatedpython");
        init();
    }
    private static native void init();
    public static native void createPythonScope();
    public static native void closePythonScope();
    public static native synchronized int performOperation(String operation);
    public static native synchronized Object operateAndExtract(String operation, String type);
    public static native synchronized void printPythonStackTrace(int errorIndex);
}