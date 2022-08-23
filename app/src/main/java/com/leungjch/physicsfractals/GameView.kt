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
    private val thread: GameThread
    private val SCALEFACTOR = 16
    private lateinit var background : Bitmap
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
        println(width.toString() + " " + height.toString())
        val conf = Bitmap.Config.ARGB_8888 // see other conf types
        background = Bitmap.createBitmap(width, height, conf)

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

    private fun renderFractalIteratively() {

        GlobalScope.launch {
            (1..32 step 2).map {
                val imageWidth = 8 * it;
                val imageHeight = 8 * it;
                val bitmapData = RustUniverse.generateFractal(
                    imageWidth,
                    imageHeight,
                    0.0,
                    0.0,
                    1.0
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
        println(event!!.action)
        when (event!!.action) {
            MotionEvent.ACTION_MOVE, MotionEvent.ACTION_DOWN-> {
                val pos = Vec2D(event.getX().toDouble()/width*RustUniverse.getWidth(), event.getY().toDouble()/height*RustUniverse.getHeight())
                RustUniverse.spawnPendulum(pos.x, pos.y, 0.001, 0.1, 1.0)
            }
            MotionEvent.ACTION_UP -> {
//                println("Getting fractal")
                        renderFractalIteratively()
//                println("DONE PARSE")
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
        val paint = Paint()
        paint.setStyle(Paint.Style.FILL)
        paint.setColor(Color.WHITE)

        for (magnet in magnets) {
            val pos: Vec2D = Utils.universeToGameCoords(magnet.pos, canvas.width, canvas.height)
            // Scale the radius
            canvas.drawCircle(pos.x.toFloat(), pos.y.toFloat(), magnet.radius.toFloat()*SCALEFACTOR, paint )
        }

        for (pendulum in pendulums) {
            val pos: Vec2D = Utils.universeToGameCoords(pendulum.pos, canvas.width, canvas.height)
            canvas.drawCircle(pos.x.toFloat(), pos.y.toFloat(),5.0.toFloat(), paint )
        }
        canvas.drawBitmap(background, 0.0.toFloat(), 0.0.toFloat(), paint)
    }

}