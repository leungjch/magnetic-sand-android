package com.leungjch.physicsfractals

import android.content.DialogInterface
import android.graphics.Color
import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.widget.Button
import android.widget.Toast
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.isVisible
import com.google.android.material.slider.Slider
import kotlin.random.Random


class MainActivity : AppCompatActivity() {
    private lateinit var gameView : GameView
    private lateinit var placementDialog: AlertDialog
    private lateinit var settingsDialog: AlertDialog

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
        val rainButton : Button = findViewById(R.id.rainButton)

        resetButton.setOnClickListener{
            RustUniverse.clearAndSpawnRandomMagnets(Random.nextInt(1, gameView.randomize_magnets_n));
            gameView.renderFractalIteratively()
        }

        loadButton.setOnClickListener{
            showRandomizeDialog()
        }

        placementButton.setOnClickListener {
            showPlacementDialog()
        }

        rainButton.setOnClickListener {
            if (gameView.isRain) {
                gameView.rain(10)
                rainButton.setBackgroundColor(Color.WHITE)
                rainButton.text = "RAIN ON"
            } else {
                gameView.rain(0)
                rainButton.setBackgroundColor(0xFFFF0000.toInt())
                rainButton.text = "RAIN OFF"
            }
            gameView.isRain = !gameView.isRain

        }

        displayButton.setOnClickListener {
            when (gameView.displayType) {
                GameView.DISPLAY_TYPES.ALL -> {
                  gameView.displayType = GameView.DISPLAY_TYPES.BACKGROUND_ONLY
                    Toast.makeText(applicationContext,
                        "Showing background only", Toast.LENGTH_SHORT).show()
                    displayButton.text = "BG ONLY"
              }
                GameView.DISPLAY_TYPES.BACKGROUND_ONLY -> {
                    gameView.displayType = GameView.DISPLAY_TYPES.NO_BACKGROUND
                    Toast.makeText(applicationContext,
                        "Showing no background", Toast.LENGTH_SHORT).show()
                    displayButton.text = "NO BG"
                }

                GameView.DISPLAY_TYPES.NO_BACKGROUND -> {
                    gameView.displayType = GameView.DISPLAY_TYPES.ALL
                    Toast.makeText(applicationContext,
                        "Showing everything", Toast.LENGTH_SHORT).show()
                    displayButton.text = "ALL"
                }
            }
        }
        optionButton.setOnClickListener {
            showSettingsDialog()
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

        var values : Array<String>  = arrayOf("Double" , "Triple",  "Quadruple", "Blank")
        builder.setSingleChoiceItems(values, -1,
            DialogInterface.OnClickListener { dialog, item ->
                gameView.clearAll()
                when (item) {
                    GameView.PRESET_TYPES.DOUBLE.ordinal -> {
                        RustUniverse.spawnMagnet(16.0, 16.0, 0.5, 2.50)
                        RustUniverse.spawnMagnet(48.0, 48.0, 0.5, 2.50)
                    }
                    GameView.PRESET_TYPES.TRIPLE.ordinal -> {
                        RustUniverse.spawnMagnet(16.0, 32.0, 0.5, 2.50)
                        RustUniverse.spawnMagnet(32.0, 16.0, 0.5, 2.50)
                        RustUniverse.spawnMagnet(48.0, 32.0, 0.5, 2.50)
                    }
                    GameView.PRESET_TYPES.QUADRUPLE.ordinal -> {
                        RustUniverse.spawnMagnet(16.0, 16.0, 0.5, 2.50)
                        RustUniverse.spawnMagnet(16.0, 48.0, 0.5, 2.50)
                        RustUniverse.spawnMagnet(48.0, 16.0, 0.5, 2.50)
                        RustUniverse.spawnMagnet(48.0, 48.0, 0.5, 2.50)
                    }
                    GameView.PRESET_TYPES.BLANK.ordinal -> {
                        RustUniverse.clearAll();
                    }
                }
                gameView.renderFractalIteratively()
            })
        builder.show()
    }

    fun showSettingsDialog() {
        var builder = AlertDialog.Builder(this)
        builder.setTitle("Settings")
        val inflater = this.getSystemService(LAYOUT_INFLATER_SERVICE) as LayoutInflater
        val dialogView: View = inflater.inflate(R.layout.settings_dialog, null)

        val stepsSlider: Slider = dialogView.findViewById(R.id.stepsSeeker)
        val qualitySlider: Slider = dialogView.findViewById(R.id.qualitySeeker)

        stepsSlider.value = gameView.steps.toFloat()
        qualitySlider.value = gameView.maxItersFactor.toFloat()

        stepsSlider.addOnChangeListener { slider, value, fromUser ->
            RustUniverse.setSteps(value.toInt())
            gameView.steps = value.toInt()

        }
        stepsSlider.addOnSliderTouchListener(object : Slider.OnSliderTouchListener{
            override fun onStartTrackingTouch(slider: Slider) {
//                // Responds to when slider's touch event is being started
            }
            override fun onStopTrackingTouch(slider: Slider) {
                // Responds to when slider's touch event is being stopped
                gameView.renderFractalIteratively()
            }
        })
        qualitySlider.addOnChangeListener { slider, value, fromUser ->
            gameView.maxItersFactor = value.toInt()
        }
        qualitySlider.addOnSliderTouchListener(object : Slider.OnSliderTouchListener{
            override fun onStartTrackingTouch(slider: Slider) {
//                // Responds to when slider's touch event is being started
            }
            override fun onStopTrackingTouch(slider: Slider) {
                // Responds to when slider's touch event is being stopped
                gameView.renderFractalIteratively()
            }
        })

        builder.setView(dialogView)
        settingsDialog = builder.create()
        settingsDialog.show()

    }

    fun showPlacementDialog() {
        var builder = AlertDialog.Builder(this)
        builder.setTitle("Set object type")
        var values : Array<String>  = arrayOf("Sand" , "Magnet",  "Emitter")
        val placementButton : Button = findViewById(R.id.placementButton)
        val inflater = this.getSystemService(LAYOUT_INFLATER_SERVICE) as LayoutInflater
        val dialogView: View = inflater.inflate(R.layout.options_dialog, null)

        val magnetOptions : View = dialogView.findViewById(R.id.magnetOptions)
        val particleOptions : View = dialogView.findViewById(R.id.particleOptions)
        val emitterOptions : View = dialogView.findViewById(R.id.emitterOptions)
        magnetOptions.isVisible=gameView.placementType==GameView.PLACEMENT_TYPES.MAGNET
        particleOptions.isVisible=gameView.placementType==GameView.PLACEMENT_TYPES.SAND
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

        val emitterSlider : Slider = dialogView.findViewById(R.id.emitterSeeker)
        emitterSlider.value = gameView.emitter_interval.toFloat()

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

        emitterSlider.addOnChangeListener { slider, value, fromUser ->
            gameView.emitter_interval = value.toInt()
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
                    GameView.PLACEMENT_TYPES.SAND.ordinal -> {
                        gameView.placementType = GameView.PLACEMENT_TYPES.SAND
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
