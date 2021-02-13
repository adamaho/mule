use indoc::formatdoc;
use std::default::Default;

use super::CSS;

/// All possible font weights
#[derive(Debug)]
pub enum FontWeight {
    Heavy,
    Bold,
    Semibold,
    Medium,
    Regular,
    Light,
    Thin,
    Ultralight,
}

// default font weight
impl Default for FontWeight {
    fn default() -> FontWeight {
        FontWeight::Regular
    }
}

// convert font weight to css
impl CSS for FontWeight {
    fn css(&self) -> String {
        match *self {
            FontWeight::Heavy => String::from("font-weight: 800;"),
            FontWeight::Bold => String::from("font-weight: 700;"),
            FontWeight::Semibold => String::from("font-weight: 600;"),
            FontWeight::Medium => String::from("font-weight: 500;"),
            FontWeight::Regular => String::from("font-weight: 400;"),
            FontWeight::Light => String::from("font-weight: 300;"),
            FontWeight::Thin => String::from("font-weight: 200;"),
            FontWeight::Ultralight => String::from("font-weight: 200;"),
        }
    }
}

// all font values
#[derive(Debug)]
pub struct Font {
    pub size: f32,
    pub weight: FontWeight,
    pub family: String,
    pub design: String,
}

// convert font to css
impl CSS for Font {
    fn css(&self) -> String {
        let css = formatdoc! {r#"
            font-size: {}rem;
            {}
            font-family: {}, {};
        "#,
        self.size,
        self.weight.css(),
        self.family,
        self.design
        };
        String::from(css)
    }
}
