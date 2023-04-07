package com.aisaiahharvey.randomcolorgenerator

class Rgb (val ptr: Long) {
    var red: Int = 0
        get() = getRed(ptr)
    var green: Int = 0
        get() = getGreen(ptr)
    var blue: Int = 0
        get() = getBlue(ptr)

    private external fun getRed(ptr: Long): Int

    private external fun getGreen(ptr: Long): Int

    private external fun getBlue(ptr: Long): Int

    companion object {
        // Used to load the 'random_color_generator' library on application startup.
        init {
            System.loadLibrary("random_color_generator")
        }

        @JvmStatic
        external fun destroyStruct(ptr: Long)
    }
}