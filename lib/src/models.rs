use rand::{distributions::Standard, prelude::Distribution, Rng};
use std::{format, string::String};

#[derive(Debug, Clone)]
pub enum Color {
    Hex(String),
    Rgb {
        red: u8,
        green: u8,
        blue: u8,
    },
    #[cfg(target_arch = "wasm32")]
    Word(String),
}

impl Color {
    pub fn random_hex<R: ?Sized + Rng>(rng: &mut R) -> Color {
        Color::Hex(format!(
            "{:x}{:x}{:x}",
            rng.gen::<u8>(),
            rng.gen::<u8>(),
            rng.gen::<u8>()
        ))
    }

    pub fn random_rgb<R: ?Sized + Rng>(rng: &mut R) -> Color {
        Color::Rgb {
            red: rng.gen::<u8>(),
            green: rng.gen::<u8>(),
            blue: rng.gen::<u8>(),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn random_word<R: ?Sized + Rng>(rng: &mut R) -> Color {
        let colors = [
            // Red Colors
            "lightsalmon",
            "salmon",
            "darksalmon",
            "lightcoral",
            "indianred",
            "crimson",
            "firebrick",
            "red",
            "darkred",
            // Orange colors
            "coral",
            "tomato",
            "orangered",
            "gold",
            "orange",
            "darkorange",
            // Yellow colors
            "lightyellow",
            "lemonchiffon",
            "lightgoldenrodyellow",
            "papayawhip",
            "moccasin",
            "peachpuff",
            "palegoldenrod",
            "khaki",
            "darkkhaki",
            "yellow",
            // Green colors
            "lawngreen",
            "chartreuse",
            "limegreen",
            "lime",
            "forestgreen",
            "green",
            "darkgreen",
            "greenyellow",
            "yellowgreen",
            "springgreen",
            "mediumspringgreen",
            "lightgreen",
            "palegreen",
            "darkseagreen",
            "mediumseagreen",
            "seagreen",
            "olive",
            "darkolivegreen",
            "olivedrab",
            // Cyan colors
            "lightcyan",
            "cyan",
            "aqua",
            "aquamarine",
            "mediumaquamarine",
            "paleturquoise",
            "turquoise",
            "mediumturquoise",
            "darkturquoise",
            "lightseagreen",
            "cadetblue",
            "darkcyan",
            "teal",
            // Blue colors
            "powderblue",
            "lightblue",
            "lightskyblue",
            "skyblue",
            "deepskyblue",
            "lightsteelblue",
            "dodgerblue",
            "cornflowerblue",
            "steelblue",
            "royalblue",
            "blue",
            "mediumblue",
            "darkblue",
            "navy",
            "midnightblue",
            "mediumslateblue",
            "slateblue",
            "darkslateblue",
            // Purple colors
            "lavender",
            "thistle",
            "plum",
            "violet",
            "orchid",
            "fuchsia",
            "magenta",
            "mediumorchid",
            "mediumpurple",
            "blueviolet",
            "darkviolet",
            "darkorchid",
            "darkmagenta",
            "purple",
            "indigo",
            // Pink colors
            "pink",
            "lightpink",
            "hotpink",
            "deeppink",
            "palevioletred",
            "mediumvioletred",
            // White colors
            "white",
            "snow",
            "honeydew",
            "mintcream",
            "azure",
            "aliceblue",
            "ghostwhite",
            "whitesmoke",
            "seashell",
            "beige",
            "oldlace",
            "floralwhite",
            "ivory",
            "antiquewhite",
            "linen",
            "lavenderblush",
            "mistyrose",
            // Gray colors
            "gainsboro",
            "lightgray",
            "silver",
            "darkgray",
            "gray",
            "dimgray",
            "lightslategray",
            "slategray",
            "darkslategray",
            "black",
            // Brown colors
            "cornsilk",
            "blanchedalmond",
            "bisque",
            "navajowhite",
            "wheat",
            "burlywood",
            "tan",
            "rosybrown",
            "sandybrown",
            "goldenrod",
            "peru",
            "chocolate",
            "saddlebrown",
            "sienna",
            "brown",
            "maroon",
        ];

        Color::Word(colors[rng.gen_range(0..colors.len())].to_owned())
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        let index = rng.gen_range(if cfg!(target_arch = "wasm32") {
            0..3
        } else {
            0..2
        });
        match index {
            0 => Color::random_hex(rng),
            1 => Color::random_rgb(rng),
            #[cfg(target_arch = "wasm32")]
            2 => Color::random_word(rng),
            _ => unreachable!(),
        }
    }
}
