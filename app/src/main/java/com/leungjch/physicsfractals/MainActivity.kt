package com.leungjch.physicsfractals

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import android.util.Log

class MainActivity : AppCompatActivity() {
    companion object {
        init {
            System.loadLibrary("cargo")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val g = RustGreetings()
        val ru = RustUniverse()
        val r = g.sayHello("cargo!")
        val textView: TextView = findViewById(R.id.greetingField) as TextView
        textView.text = r
        val universeWidth = ru.getWidth()
        val universeHeight = ru.getHeight()
        println(universeWidth.toString())

        ru.createMagnet((universeWidth/2.0).toDouble(), (universeHeight/2.0).toDouble(), 10.0, 10.0)
        val x = ru.getMagnets()
        println(x[0])

    }
}