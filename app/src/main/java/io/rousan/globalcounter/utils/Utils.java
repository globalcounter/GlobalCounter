package io.rousan.globalcounter.utils;

import android.util.Log;

public final class Utils {
    public static void log(Object msg) {
        Log.d("global-counter", String.format("Java: %s", msg));
    }
}