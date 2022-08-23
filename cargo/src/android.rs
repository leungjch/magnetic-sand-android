#![allow(non_snake_case)]

use crate::Universe;
use std::ffi::{CStr, CString};
/// Expose the JNI interface for android below
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
use std::os::raw::c_char;

pub mod android {
    extern crate jni;

    use crate::generate_fractal;

    use self::jni::objects::{JClass, JString, JObject};
    use self::jni::sys::{jbyteArray, jint, jlong, jstring, jdouble, jobject};
    use self::jni::JNIEnv;
    use super::*;

    static mut UNIVERSE: Universe = Universe {
        width: 64,
        height: 64,
        nums: vec![],
        pendulums: vec![],
        magnets: vec![],
        emitters: vec![],
        steps: 100,
        delta: 0.01,
        max_iters: 500,
    };


    pub extern "C" fn rust_greeting(to: *const c_char) -> *mut c_char {
        let c_str = unsafe { CStr::from_ptr(to) };
        let recipient = match c_str.to_str() {
            Err(_) => "there",
            Ok(string) => string,
        };

        CString::new("Helloooopppo ".to_owned() + recipient)
            .unwrap()
            .into_raw()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustGreetings_greeting(
        env: JNIEnv,
        _: JClass,
        java_pattern: JString,
    ) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let world = rust_greeting(
            env.get_string(java_pattern)
                .expect("invalid pattern string")
                .as_ptr(),
        );
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let world_ptr = CString::from_raw(world);
        let output = env
            .new_string(world_ptr.to_str().unwrap())
            .expect("Couldn't create java string!");

        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rGetWidth(
        env: JNIEnv,
        _class: JClass,
    ) -> jlong {
        UNIVERSE.width as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rGetHeight(
        env: JNIEnv,
        _class: JClass,
    ) -> jlong {
        UNIVERSE.height as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rTick(
        env: JNIEnv,
        _class: JClass,
    ) {
        UNIVERSE.tick();
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rCreateMagnet(
        env: JNIEnv,
        _class: JClass,
        x: jdouble,
        y: jdouble,
        strength: jdouble,
        radius: jdouble,
    ) {
        UNIVERSE.create_magnet(
            x as f64,
            y as f64,
            strength as f64,
            radius as f64
        );
    }


    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rGetMagnetsBytes(
        env: JNIEnv,
        _class: JClass,
    ) -> jbyteArray {
        let magnet = UNIVERSE.magnets_to_u8();
        let output = env.byte_array_from_slice(magnet.as_slice()).unwrap();
        output
    }


    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rGetPendulumsBytes(
        env: JNIEnv,
        _class: JClass,
    ) -> jbyteArray {
        let pendulums = UNIVERSE.pendulums_to_u8();
        let output = env.byte_array_from_slice(pendulums.as_slice()).unwrap();
        output
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rSpawnPendulum(
        env: JNIEnv,
        _class: JClass,
        x: jdouble,
        y: jdouble,
        tension: jdouble,
        friction: jdouble,
        mass: jdouble
    ) {
       UNIVERSE.create_pendulum(x as f64, y as f64, tension as f64, friction as f64, mass as f64) 
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rSpawnMagnet(
        env: JNIEnv,
        _class: JClass,
        x: jdouble,
        y: jdouble,
        strength: jdouble,
        radius: jdouble
    ) {
       UNIVERSE.create_magnet(x as f64, y as f64, strength as f64, radius as f64) 
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rSpawnRandomEmitters(
        env: JNIEnv,
        _class: JClass,
        n: jint,
        tension: jdouble,
        friction: jdouble,
        mass: jdouble
    ) {
       UNIVERSE.spawn_random_emitters(n as u32, tension as f64, friction as f64, mass as f64) 
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rClearAndSpawnRandomMagnets(
        env: JNIEnv,
        _class: JClass,
        n: jint,
    ) {
       UNIVERSE.clear_and_spawn_random_magnets(n as u32) 
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rClearAll(
        env: JNIEnv,
        _class: JClass,
    ) {
       UNIVERSE.clear_all()
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_leungjch_physicsfractals_RustUniverse_rGenerateFractal(
        env: JNIEnv,
        _class: JClass,
        image_width: jint,
        image_height: jint,
        tension: jdouble,
        friction: jdouble,
        mass: jdouble,
    ) -> jbyteArray {
        let rgbVec : Vec<u8> = generate_fractal(image_width as usize, image_height as usize, &UNIVERSE, tension as f64, friction as f64, mass as f64);
        let output = env.byte_array_from_slice(rgbVec.as_slice()).unwrap();
        output

    }

}
