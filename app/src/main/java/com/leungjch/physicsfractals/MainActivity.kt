package com.leungjch.physicsfractals

import android.content.DialogInterface
import android.content.Intent
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.widget.Button
import android.widget.LinearLayout
import android.widget.SeekBar
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.isVisible
import com.google.android.material.slider.Slider


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
        val placementButton : Button = findViewById(R.id.placementButton)
        val displayButton : Button = findViewById(R.id.displayButton)
        val optionButton : Button = findViewById(R.id.optionButton)
        resetButton.setOnClickListener{
            gameView.clearAll()
            Toast.makeText(applicationContext, "hi", Toast.LENGTH_SHORT).show()
        }

        loadButton.setOnClickListener{
            showRandomizeDialog()
        }

        placementButton.setOnClickListener {
            showPlacementDialog()
        }

        displayButton.setOnClickListener {
            println("HIII")
            when (gameView.displayType) {
                GameView.DISPLAY_TYPES.ALL -> {
                  gameView.displayType = GameView.DISPLAY_TYPES.BACKGROUND_ONLY
              }
                GameView.DISPLAY_TYPES.BACKGROUND_ONLY -> {
                    gameView.displayType = GameView.DISPLAY_TYPES.NO_BACKGROUND
                }

                GameView.DISPLAY_TYPES.NO_BACKGROUND -> {
                    gameView.displayType = GameView.DISPLAY_TYPES.ALL
                }
            }
        }
        optionButton.setOnClickListener {
        }



    }

    fun showRandomizeDialog() {
        val builder = AlertDialog.Builder(this)
        builder.setTitle("Load preset")
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
            gameView.clearAndRandomize(3)
        }
        builder.show()
    }

    fun showPlacementDialog() {
        var builder = AlertDialog.Builder(this)
        builder.setTitle("Set placement type")
        var values : Array<String>  = arrayOf("Particle" , "Magnet",  "Emitter")
        val placementButton : Button = findViewById(R.id.placementButton)
        val inflater = this.getSystemService(LAYOUT_INFLATER_SERVICE) as LayoutInflater
        val dialogView: View = inflater.inflate(R.layout.options_dialog, null)

        val magnetOptions : View = dialogView.findViewById(R.id.magnetOptions)
        val particleOptions : View = dialogView.findViewById(R.id.particleOptions)
        val emitterOptions : View = dialogView.findViewById(R.id.emitterOptions)
        magnetOptions.isVisible=gameView.placementType==GameView.PLACEMENT_TYPES.MAGNET
        particleOptions.isVisible=gameView.placementType==GameView.PLACEMENT_TYPES.PARTICLE
        emitterOptions.isVisible=gameView.placementType==GameView.PLACEMENT_TYPES.EMITTER

//        Set slider defaults
        val tensionSlider: Slider = dialogView.findViewById(R.id.tensionSeeker)
        val frictionSlider: Slider = dialogView.findViewById(R.id.frictionSeeker)
        val massSlider: Slider = dialogView.findViewById(R.id.massSeeker)
        tensionSlider.value = gameView.pendulum_tension.toFloat()
        frictionSlider.value = gameView.pendulum_friction.toFloat()
        massSlider.value = gameView.pendulum_mass.toFloat()

        val strengthSlider : Slider = dialogView.findViewById(R.id.strengthSeeker)
        val radiusSeeeker : Slider = dialogView.findViewById(R.id.radiusSeeker)
        strengthSlider.value = gameView.magnet_strength.toFloat()
        radiusSeeeker.value = gameView.magnet_radius.toFloat()

        tensionSlider.addOnChangeListener { slider, value, fromUser ->
            gameView.pendulum_tension = value.toDouble()
        }
        tensionSlider.addOnSliderTouchListener(object : Slider.OnSliderTouchListener{
            override fun onStartTrackingTouch(slider: Slider) {
//                // Responds to when slider's touch event is being started
            }
            override fun onStopTrackingTouch(slider: Slider) {
                // Responds to when slider's touch event is being stopped
                gameView.renderFractalIteratively()
            }
        })
        frictionSlider.addOnChangeListener { slider, value, fromUser ->
            gameView.pendulum_friction = value.toDouble()
        }
        frictionSlider.addOnSliderTouchListener(object : Slider.OnSliderTouchListener{
            override fun onStartTrackingTouch(slider: Slider) {
//                // Responds to when slider's touch event is being started
            }
            override fun onStopTrackingTouch(slider: Slider) {
                // Responds to when slider's touch event is being stopped
                gameView.renderFractalIteratively()
            }
        })
        massSlider.addOnChangeListener { slider, value, fromUser ->
            gameView.pendulum_mass = value.toDouble()
        }
        massSlider.addOnSliderTouchListener(object : Slider.OnSliderTouchListener{
            override fun onStartTrackingTouch(slider: Slider) {
//                // Responds to when slider's touch event is being started
            }
            override fun onStopTrackingTouch(slider: Slider) {
                // Responds to when slider's touch event is being stopped
                gameView.renderFractalIteratively()
            }
        })
        radiusSeeeker.addOnChangeListener { slider, value, fromUser ->
            gameView.magnet_radius = value.toDouble()
        }
        strengthSlider.addOnChangeListener { slider, value, fromUser ->
            gameView.magnet_strength = value.toDouble()
        }

        builder.setSingleChoiceItems(values, gameView.placementType.ordinal,
            DialogInterface.OnClickListener { dialog, item ->
                when (item) {
                    GameView.PLACEMENT_TYPES.MAGNET.ordinal -> {
                        gameView.placementType = GameView.PLACEMENT_TYPES.MAGNET
                        magnetOptions.isVisible=true
                        particleOptions.isVisible=false
                        emitterOptions.isVisible=false
                    }
                    GameView.PLACEMENT_TYPES.PARTICLE.ordinal -> {
                        gameView.placementType = GameView.PLACEMENT_TYPES.PARTICLE
                        magnetOptions.isVisible=false
                        particleOptions.isVisible=true
                        emitterOptions.isVisible=false

                    }
                    GameView.PLACEMENT_TYPES.EMITTER.ordinal -> {
                        gameView.placementType = GameView.PLACEMENT_TYPES.EMITTER
                        magnetOptions.isVisible=false
                        particleOptions.isVisible=false
                        emitterOptions.isVisible=true
                    }
                }
                placementButton.text = gameView.placementType.toString()

            })
        builder.setView(dialogView)
        placementDialog = builder.create()
        placementDialog.show()

    }


}
