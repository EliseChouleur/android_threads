package com.example.jnitestapplication;

import android.content.Context;

public class JniInterface {
    static {
        System.loadLibrary("jni_test_rust_lib");
    }

    public static native void runRustExample(Context context);
}
