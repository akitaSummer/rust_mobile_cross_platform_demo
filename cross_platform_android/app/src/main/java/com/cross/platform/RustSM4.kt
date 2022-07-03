package com.cross.platform

class RustSM4 {
    companion object {
        init {
            System.loadLibrary("android_code")
        }
    }

    private external fun sm4(pattern: String): String?

    fun my_sm4(to: String): String {
        return sm4(to) ?: "error"
    }
}