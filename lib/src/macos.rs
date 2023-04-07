#![allow(non_snake_case)]

use rand::thread_rng;
use std::{
    ffi::{c_char, CStr, CString},
    fmt::Display,
    format,
};

use crate::models::Color;

#[repr(C)]
#[derive(Debug, PartialEq, PartialOrd, Default)]
pub struct Rgb {
    red: f64,
    green: f64,
    blue: f64,
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(value) => write!(f, "#{value}"),
            Color::Rgb { red, green, blue } => write!(f, "({red},{green},{blue})"),
        }
    }
}

/// This generates a random `Rgb` object
#[no_mangle]
pub extern "C" fn randomRgb() -> Rgb {
    let mut rng = thread_rng();
    if let Color::Rgb { red, green, blue } = Color::random_rgb(&mut rng) {
        Rgb {
            red: red as f64 / 255.,
            green: green as f64 / 255.,
            blue: blue as f64 / 255.,
        }
    } else {
        Rgb::default()
    }
}

#[no_mangle]
pub extern "C" fn randomHex() -> *const c_char {
    let mut rng = thread_rng();
    let s = format!("#{}", Color::random_hex(&mut rng));
    let c_str = CString::new(s).expect("Could not create C string from Rust string");
    c_str.as_ptr()
}

/// This function frees a Rust created C String
///
/// # Safety
///
/// This function dereferences a pointer
#[no_mangle]
pub unsafe extern "C" fn freeRustCString(string: *const c_char) {
    let _ = CStr::from_ptr(string);
}
