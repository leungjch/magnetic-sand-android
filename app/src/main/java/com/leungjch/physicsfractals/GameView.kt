package com.leungjch.physicsfractals


import android.content.Context
import android.graphics.*
import android.util.AttributeSet
import android.view.MotionEvent
import android.view.SurfaceHolder
import android.view.SurfaceView
import java.nio.ByteBuffer
import androidx.lifecycle.lifecycleScope
import kotlinx.coroutines.*


class GameView(context: Context, attributes: AttributeSet) : SurfaceView(context, attributes), SurfaceHolder.Callback {
    enum class PLACEMENT_TYPES {
        PARTICLE, MAGNET, EMITTER
    }

    enum class DISPLAY_TYPES {
        ALL, BACKGROUND_ONLY, NO_BACKGROUND
    }

    private val thread: GameThread
    private val SCALEFACTOR = 16
    private lateinit var background : Bitmap
    public var placementType = PLACEMENT_TYPES.PARTICLE
    public var pendulum_friction : Double = 0.0
    public var pendulum_tension : Double = 0.0
    public var pendulum_mass : Double = 1.0
    public var magnet_strength : Double = 0.1
    public var magnet_radius : Double = 1.0
    public var emitter_interval : Int = 2
    public var randomize_magnets_n : Int = 3
    public var displayType = DISPLAY_TYPES.ALL
    private var generation_job = GlobalScope.launch {}
    init {

        // add callback
        holder.addCallback(this)

        // instantiate the game thread
        thread = GameThread(holder, this)

    }


    override fun surfaceCreated(surfaceHolder: SurfaceHolder) {
        // start the game thread
        thread.setRunning(true)
        thread.start()
    }

    override fun surfaceChanged(surfaceHolder: SurfaceHolder, i: Int, width: Int, height: Int) {
        println("height is" + width.toString() + " " + height.toString())
        val conf = Bitmap.Config.ARGB_8888 // see other conf types
        background = Bitmap.createBitmap(width, height, conf)
//        RustUniverse.setWidth(width)
//        RustUniverse.setHeight(height)
        renderFractalIteratively()

    }

    override fun surfaceDestroyed(surfaceHolder: SurfaceHolder) {
        var retry = true
        while (retry) {
            try {
                thread.setRunning(false)
                thread.join()
            } catch (e: Exception) {
                e.printStackTrace()
            }

            retry = false
        }
    }

    public fun renderFractalIteratively() {
        println("Generation job is")
        println(generation_job != null)
//      If we are already generating a fractal previously, cancel it
        if (generation_job != null) {
            generation_job?.cancel()
        }
        generation_job = GlobalScope.launch {
            (1..32 step 4).map {
                println("Step " + it.toString())
                val imageWidth = 8 * it;
                val imageHeight = 8 * it;
                val bitmapData = RustUniverse.generateFractal(
                    imageWidth,
                    imageHeight,
                    pendulum_tension,
                    pendulum_friction,
                    pendulum_mass,
                )
                val bmp = Bitmap.createBitmap(
                    imageWidth,
                    imageHeight,
                    Bitmap.Config.ARGB_8888
                )
                val buffer: ByteBuffer = ByteBuffer.wrap(bitmapData)
                bmp.copyPixelsFromBuffer(buffer)
                background =
                    Bitmap.createScaledBitmap(bmp, width, height, false)
            }
        }
    }



    override fun onTouchEvent(event: MotionEvent?): Boolean {
        val pos = Vec2D(event!!.getX().toDouble()/width*RustUniverse.getWidth(), event.getY().toDouble()/height*RustUniverse.getHeight())
        when (event!!.action) {
            MotionEvent.ACTION_MOVE -> {
                when (placementType) {
                    PLACEMENT_TYPES.PARTICLE -> {
                        RustUniverse.spawnPendulum(pos.x, pos.y, pendulum_tension, pendulum_friction, pendulum_mass)
                    }
                    else -> {}
                }
            }
            MotionEvent.ACTION_DOWN -> {

                when (placementType) {
                    PLACEMENT_TYPES.PARTICLE -> {
                        RustUniverse.spawnPendulum(pos.x, pos.y, pendulum_tension, pendulum_friction, pendulum_mass)
                    }
                    PLACEMENT_TYPES.MAGNET -> {
                        RustUniverse.spawnMagnet(pos.x, pos.y, magnet_strength, magnet_radius)
                        renderFractalIteratively()
                    }
                    PLACEMENT_TYPES.EMITTER -> {
                        RustUniverse.spawnEmitter(pos.x, pos.y, emitter_interval, pendulum_tension, pendulum_friction, pendulum_mass)
                    }

                }
            }
            MotionEvent.ACTION_UP -> {

            }
        }
        return true
    }
    /**
     * Function to update the positions of player and game objects
     */
    fun update() {
        // Update the universe
        RustUniverse.tick()
    }

    /**
     * Everything that has to be drawn on Canvas
     */
    override fun draw(canvas: Canvas) {

        super.draw(canvas)
//      Draw the fractal background
        val magnets = RustUniverse.getMagnets()
        val pendulums = RustUniverse.getPendulums()
//        val emitters = RustUniverse.getEmitters()
        val paint = Paint()
        paint.setStyle(Paint.Style.FILL)
        paint.setColor(Color.WHITE)

//      Do conditional rendering based on the display type selected
        if (displayType == DISPLAY_TYPES.ALL || displayType == DISPLAY_TYPES.BACKGROUND_ONLY) {
            canvas.drawBitmap(background, 0.0.toFloat(), 0.0.toFloat(), paint)
        }
        if (displayType == DISPLAY_TYPES.ALL || displayType == DISPLAY_TYPES.NO_BACKGROUND) {
            for (magnet in magnets) {
                val pos: Vec2D = Utils.universeToGameCoords(magnet.pos, canvas.width, canvas.height)
                // Scale the radius
                val magnetPaint = Paint()
                val magnetColor = Color.rgb((magnet.color.r+255)/2, (magnet.color.g+255)/2, (magnet.color.b+255)/2)
                magnetPaint.setStyle(Paint.Style.FILL)
                magnetPaint.setColor(magnetColor)
                canvas.drawCircle(pos.x.toFloat(), pos.y.toFloat(), magnet.radius.toFloat()*SCALEFACTOR, magnetPaint )
            }
        }

        if (displayType == DISPLAY_TYPES.ALL || displayType == DISPLAY_TYPES.NO_BACKGROUND) {
            for (pendulum in pendulums) {
                val pos: Vec2D =
                    Utils.universeToGameCoords(pendulum.pos, canvas.width, canvas.height)
                canvas.drawCircle(pos.x.toFloat(), pos.y.toFloat(), 5.0.toFloat(), paint)
            }
        }

//        for (emitter in emitters) {
//            val pos: Vec2D = Utils.universeToGameCoords(pendulum.pos, canvas.width, canvas.height)
//            canvas.drawCircle(pos.x.toFloat(), pos.y.toFloat(),5.0.toFloat(), paint )
//        }
    }

    fun clearAll() {
        RustUniverse.clearAll()
    }

    fun clearAndRandomize(n: Int) {
        RustUniverse.clearAndSpawnRandomMagnets(n)
        renderFractalIteratively()
    }

}