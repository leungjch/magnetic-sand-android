package com.leungjch.physicsfractals

import kotlin.math.*

data class Rgb(val r: Int, val g: Int, val b: Int)
data class Vec2D(val x: Double, val y: Double)
data class Magnet(val pos: Vec2D, val strength: Double, val radius: Double, val color: Rgb )
// Contains too many unnecessary fields
//data class Pendulum(val pos: Vec2D, val vel: Vec2D, val acc: Vec2D, val mass: Double, val f_tension: Vec2D, val friction: Double, val f_magnetic: Vec2D, val f_net: Vec2D, val is_stationary: Boolean, val magnet_color: Rgb)
data class Pendulum(val pos: Vec2D, val vel: Vec2D, val acc: Vec2D, val mass: Double)

class Utils {
    companion object {
        public fun universeToGameCoords(
            pos: Vec2D,
            gameWidth: Int,
            gameHeight: Int,
        ): Vec2D {
            return Vec2D(
                pos.x / RustUniverse.getWidth().toDouble() * gameWidth.toDouble(),
                pos.y / RustUniverse.getHeight().toDouble() * gameHeight.toDouble(),
            )
        }

        public fun scalingFactor(canvasWidth: Int, canvasHeight: Int): Float {
            val unitVectorScaled = universeToGameCoords(Vec2D(1.0,1.0), canvasWidth, canvasHeight)
            return sqrt(unitVectorScaled.x.toFloat().pow(2)+ unitVectorScaled.y.toFloat().pow(2))
        }
    }
}