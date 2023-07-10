#![allow(non_snake_case)]

use jni::{
    objects::{JClass, JObject},
    sys::jlong,
    JNIEnv,
};
use rand::thread_rng;
use std::fmt::Display;

use crate::models::Color;

#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct Rgb {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Color { red, green, blue } = self;
        write!(f, "({red},{green},{blue})")
    }
}

/// This generates a random `Rgb` object
///
/// # Safety
///
/// This is a Rust function being used in a Java environment.
#[no_mangle]
pub unsafe extern "C" fn Java_com_aisaiahharvey_randomcolorgenerator_MainActivity_randomRgb(
    _: JNIEnv,
    _: JObject,
) -> *mut Rgb {
    let mut rng = thread_rng();
    let Color { red, green, blue } = Color::random_rgb(&mut rng);
    let result = Rgb {
        red: red as i32,
        green: green as i32,
        blue: blue as i32,
    };

    Box::into_raw(Box::new(result))
}

/// # Safety
///
/// This is a Rust function being used in a Java environment.
#[no_mangle]
pub unsafe extern "C" fn Java_com_aisaiahharvey_randomcolorgenerator_Rgb_getRed(
    _: JNIEnv,
    _: JObject,
    ptr: *const Rgb,
) -> i32 {
    unsafe { (*ptr).red }
}

/// # Safety
///
/// This is a Rust function being used in a Java environment.
#[no_mangle]
pub unsafe extern "C" fn Java_com_aisaiahharvey_randomcolorgenerator_Rgb_getGreen(
    _: JNIEnv,
    _: JObject,
    ptr: *const Rgb,
) -> i32 {
    unsafe { (*ptr).green }
}

/// # Safety
///
/// This is a Rust function being used in a Java environment.
#[no_mangle]
pub unsafe extern "C" fn Java_com_aisaiahharvey_randomcolorgenerator_Rgb_getBlue(
    _: JNIEnv,
    _: JObject,
    ptr: *const Rgb,
) -> i32 {
    unsafe { (*ptr).blue }
}

/// # Safety
///
/// This is a Rust function being used in a Java environment.
#[no_mangle]
pub unsafe extern "C" fn Java_com_aisaiahharvey_randomcolorgenerator_Rgb_destroyStruct(
    _: JNIEnv,
    _: JClass,
    ptr: jlong,
) {
    let _ = Box::from_raw(ptr as *mut Rgb);
}
