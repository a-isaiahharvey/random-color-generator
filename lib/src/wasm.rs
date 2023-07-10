#![allow(non_snake_case)]

use core::fmt::Display;

use crate::models::Color;

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Color { red, green, blue } = self;
        write!(f, "rgb({red},{green},{blue})")
    }
}
