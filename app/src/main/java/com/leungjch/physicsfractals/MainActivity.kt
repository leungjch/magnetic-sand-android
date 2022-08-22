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
        println(r)
        val universeWidth = RustUniverse.getWidth()
        val universeHeight = RustUniverse.getHeight()
//        val textView: TextView = findViewById(R.id.greetingField) as TextView
//        textView.text = r
        println(universeWidth.toString())

//        RustUniverse.createMagnet((universeWidth/2.0).toDouble(), (universeHeight/2.0).toDouble(), 0.1, 2.0)
//        RustUniverse.createMagnet((universeWidth/4.0).toDouble(), (universeHeight/2.0).toDouble(), 0.1, 2.0)
        val x = RustUniverse.getMagnets()
        RustUniverse.clearAndSpawnRandomMagnets(3)
//        RustUniverse.spawnRandomEmitters(10, 0.0, 0.0, 1.0)
        val rgb = RustUniverse.generateFractal(64, 64, 1.0, 1.0, 1.0)
        println("HEllo")
        println(rgb)
    }
}