package com.aisaiahharvey.randomcolorgenerator

import android.graphics.Color
import android.os.Bundle
import android.view.View
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
    }

    fun setRandomActivityBackgroundColor(view: View) {
        val view = this.window.decorView
        val currentColorTextView: TextView = findViewById(R.id.currentColorText) as TextView
        val rgb = Rgb(randomRgb());
        view.setBackgroundColor(Color.rgb(rgb.red, rgb.green, rgb.blue))
        currentColorTextView.text = rgbToHex(rgb.red, rgb.green, rgb.blue)
        Rgb.destroyStruct(rgb.ptr)
    }

    private fun rgbToHex(red: Int, green: Int, blue: Int): String {
        return String.format("#%02x%02x%02x", red, green, blue)
    }

    /**
     * A native method that is implemented by the 'random_color_generator' native library,
     * which is packaged with this application.
     */
    private external fun randomRgb(): Long

    companion object {
        // Used to load the 'random_color_generator' library on application startup.
        init {
            System.loadLibrary("random_color_generator")
        }
    }
}