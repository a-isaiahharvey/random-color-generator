#![allow(non_snake_case)]

use core::fmt::Display;

use crate::models::Color;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Hex(hex) => write!(f, "#{hex}"),
            Color::Rgb { red, green, blue } => write!(f, "rgb({red},{green},{blue})"),
            Color::Word(word) => write!(f, "{word}"),
        }
    }
}
