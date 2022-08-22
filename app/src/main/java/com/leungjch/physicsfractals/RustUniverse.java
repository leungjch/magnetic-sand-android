package com.leungjch.physicsfractals;
import java.io.ByteArrayInputStream;
import java.io.DataInputStream;
import java.io.EOFException;
import java.io.IOException;
import java.util.ArrayList;
import java.io.DataInputStream;

import kotlin.UByte;

public class RustUniverse {

    public static native long rGetWidth();
    public static long getWidth() {
        return rGetWidth();
    }

    public static native long rGetHeight();
    public static long getHeight() {
        return rGetHeight();
    }

    public static native void rTick();
    public static void tick() {
        rTick();
    }

    public static native void rCreateMagnet(double x, double y, double strength, double radius);
    public static void createMagnet(double x, double y, double strength, double radius) {
        rCreateMagnet(x,y,strength,radius);
    }

    public static native byte[] rGetMagnetsBytes();
    public static ArrayList<Magnet> getMagnets() {
       byte[] magnets_bytes = rGetMagnetsBytes();
       ArrayList<Magnet> magnets = new ArrayList<Magnet>();
        DataInputStream dis = new DataInputStream(new ByteArrayInputStream(magnets_bytes));
        boolean eof = false;
        while (!eof) {
            try {
                Double pos_x = dis.readDouble();
                Double pos_y = dis.readDouble();
                Double strength = dis.readDouble();
                Double radius = dis.readDouble();
                Byte r = dis.readByte();
                Byte g = dis.readByte();
                Byte b = dis.readByte();
                magnets.add(new Magnet(
                        new Vec2D(pos_x, pos_y),
                        strength,
                        radius,
                        new Rgb(Byte.toUnsignedInt(r), Byte.toUnsignedInt(g), Byte.toUnsignedInt(b))
                ));
            }
            catch (EOFException e) {
                eof = true;
            }
            catch (IOException e) {
                eof = true;
            }
        }
        return magnets;
    }


    public static native byte[] rGetPendulumsBytes();
    public static ArrayList<Pendulum> getPendulums() {
        byte[] pendulum_bytes = rGetPendulumsBytes();
        ArrayList<Pendulum> pendulums = new ArrayList<Pendulum>();
        DataInputStream dis = new DataInputStream(new ByteArrayInputStream(pendulum_bytes));
        boolean eof = false;
        while (!eof) {
            try {
                Double pos_x = dis.readDouble();
                Double pos_y = dis.readDouble();
                Double vel_x = dis.readDouble();
                Double vel_y = dis.readDouble();
                Double acc_x = dis.readDouble();
                Double acc_y = dis.readDouble();
                Double mass = dis.readDouble();

                pendulums.add(new Pendulum(
                        new Vec2D(pos_x, pos_y),
                        new Vec2D(vel_x, vel_y),
                        new Vec2D(acc_x, acc_y),
                        mass
                ));
            }
            catch (EOFException e) {
                eof = true;
            }
            catch (IOException e) {
                eof = true;
            }
        }
        return pendulums;
    }

    public static native void rClearAndSpawnRandomMagnets(int n);
    public static void clearAndSpawnRandomMagnets(int n) {
        rClearAndSpawnRandomMagnets(n);
    }


    public static native void rSpawnRandomEmitters(int n, double tension, double friction, double mass);
    public static void spawnRandomEmitters(int n, double tension, double friction, double mass) {
       rSpawnRandomEmitters(n, tension, friction, mass);
    }

    public static native void rSpawnPendulum(double x, double y, double tension, double friction, double mass);
    public static void spawnPendulum(double x, double y, double tension, double friction, double mass) {
        rSpawnPendulum(x, y, tension, friction, mass);
    }

    public static native void rSpawnMagnet(double x, double y, double strength, double radius);
    public static void spawnMagnet(double x, double y, double strength, double radius) {
        rSpawnMagnet(x, y, strength, radius);
    }


    public static native byte[] rGenerateFractal(int image_width, int image_height, double tension, double friction, double mass);
    public static byte[] generateFractal(int image_width, int image_height, double tension, double friction, double mass) {
        return rGenerateFractal(image_width, image_height, tension, friction, mass);
    }
}
