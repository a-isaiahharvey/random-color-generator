use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::{format, string::String};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Color {
    pub fn random_hex<R: ?Sized + Rng>(rng: &mut R) -> String {
        Color::random(rng).to_hex()
    }

    pub fn random<R: ?Sized + Rng>(rng: &mut R) -> Color {
        Self {
            red: rng.gen::<u8>(),
            green: rng.gen::<u8>(),
            blue: rng.gen::<u8>(),
        }
    }

    pub fn from_hex(hex: &str) -> Self {
        let number = if let Some(index) = hex.find('#') {
            i32::from_str_radix(hex[index + 1..].trim(), 16)
        } else {
            i32::from_str_radix(hex.trim(), 16)
        }
        .expect("Not a hexidecimal value");

        Color {
            red: ((number & 0xFF0000) >> 16) as u8,
            green: ((number & 0x00FF00) >> 8) as u8,
            blue: (number & 0x0000FF) as u8,
        }
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Color { red, green, blue }
    }

    pub fn to_hex(&self) -> String {
        let Color { red, green, blue } = self;
        format!("{red:02X}{green:02X}{blue:02X}")
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        Color::random(rng)
    }
}

#[cfg(test)]
mod tests {
    use super::Color;

    #[test]
    fn test_from_hex() {
        let white = Color::from_hex("#FFFFFF");
        assert_eq!(white, Color::from_rgb(255, 255, 255));

        let black = Color::from_hex("#000000");
        assert_eq!(black, Color::from_rgb(0, 0, 0));

        let red = Color::from_hex("#FF0000");
        assert_eq!(red, Color::from_rgb(255, 0, 0));

        let green = Color::from_hex("#00FF00");
        assert_eq!(green, Color::from_rgb(0, 255, 0));

        let blue = Color::from_hex("#0000FF");
        assert_eq!(blue, Color::from_rgb(0, 0, 255));

        let burly_wood = Color::from_hex("#DEB887");
        assert_eq!(burly_wood, Color::from_rgb(222, 184, 135));
    }

    #[test]
    fn test_to_hex() {
        let white = Color::from_rgb(255, 255, 255).to_hex();
        assert_eq!(white, "FFFFFF");

        let black = Color::from_rgb(0, 0, 0).to_hex();
        assert_eq!(black, "000000");

        let red = Color::from_rgb(255, 0, 0).to_hex();
        assert_eq!(red, "FF0000");

        let green = Color::from_rgb(0, 255, 0).to_hex();
        assert_eq!(green, "00FF00");

        let blue = Color::from_rgb(0, 0, 255).to_hex();
        assert_eq!(blue, "0000FF");

        let burly_wood = Color::from_rgb(222, 184, 135).to_hex();
        assert_eq!(burly_wood, "DEB887");
    }
}
