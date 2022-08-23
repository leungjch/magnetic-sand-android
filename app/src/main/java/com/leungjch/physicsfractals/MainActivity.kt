package com.leungjch.physicsfractals

import android.content.DialogInterface
import android.os.Bundle
import android.widget.Button
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity


class MainActivity : AppCompatActivity() {
    private lateinit var gameView : GameView
    private lateinit var placementDialog: AlertDialog

    companion object {
        init {
            System.loadLibrary("cargo")
        }
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
//      Remove app bar
        supportActionBar?.hide()

        setContentView(R.layout.activity_main)

        gameView = findViewById(R.id.gameView);

//      Initialize rust backend
        val g = RustGreetings()
        val ru = RustUniverse()
        val r = g.sayHello("cargo!")
        println(r)
        val universeWidth = RustUniverse.getWidth()
        val universeHeight = RustUniverse.getHeight()
        println(universeWidth.toString())
        val x = RustUniverse.getMagnets()
        RustUniverse.clearAndSpawnRandomMagnets(3)

////      Setup button listeners
        val resetButton: Button = findViewById(R.id.resetButton)
        val loadButton: Button = findViewById(R.id.loadButton)

        resetButton.setOnClickListener{
            gameView.clearAll()
            Toast.makeText(applicationContext, "hi", Toast.LENGTH_SHORT).show()
        }

        loadButton.setOnClickListener{
            showPlacementDialog()
        }



    }

    fun showRandomizeDialog() {
        val builder = AlertDialog.Builder(this)
        builder.setTitle("Reset Screen")
        builder.setMessage("We have a message")
//builder.setPositiveButton("OK", DialogInterface.OnClickListener(function = x))

        builder.setPositiveButton(android.R.string.yes) { dialog, which ->
            Toast.makeText(applicationContext,
                android.R.string.yes, Toast.LENGTH_SHORT).show()
        }

        builder.setNegativeButton(android.R.string.no) { dialog, which ->
            Toast.makeText(applicationContext,
                android.R.string.no, Toast.LENGTH_SHORT).show()
        }

        builder.setNeutralButton("Randomize") { dialog, which ->
            Toast.makeText(applicationContext,
                "Randomize", Toast.LENGTH_SHORT).show()
            gameView.clearAndRandomize(10)
        }
        builder.show()

    }

    fun showPlacementDialog() {
        var builder = AlertDialog.Builder(this)
        builder.setTitle("Set placement type")
        var values : Array<String>  = arrayOf("Magnet" , "Particle" , "Emitter")

        builder.setSingleChoiceItems(values, -1,
            DialogInterface.OnClickListener { dialog, item ->
                when (item) {
                    0 -> {
                        Toast.makeText(this@MainActivity, "Selected Particle", Toast.LENGTH_LONG)
                            .show()
                        gameView.placementType = GameView.PLACEMENT_TYPES.MAGNET

                    }
                    1 -> {
                        Toast.makeText(this@MainActivity, "Selected Particle", Toast.LENGTH_LONG)
                            .show()
                        gameView.placementType = GameView.PLACEMENT_TYPES.PARTICLE
                    }
                    2 -> {
                        Toast.makeText(
                            this@MainActivity,
                            "Selected Emitter",
                            Toast.LENGTH_SHORT
                        )
                            .show()
                        gameView.placementType = GameView.PLACEMENT_TYPES.EMITTER
                    }
                }
                placementDialog.dismiss()
            })
        placementDialog = builder.create()
        placementDialog.show()

    }


}
