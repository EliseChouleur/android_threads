package com.example.myapplication;

import android.content.Context;

public class JniInterface {
    static {
        System.loadLibrary("rust_lib");
    }

    public static native void runRustExample(Context context);
}
