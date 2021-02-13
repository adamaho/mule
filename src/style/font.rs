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
        let weight = match *self {
            FontWeight::Heavy => "800",
            FontWeight::Bold => "700",
            FontWeight::Semibold => "600",
            FontWeight::Medium => "500",
            FontWeight::Regular => "400",
            FontWeight::Light => "300",
            FontWeight::Thin => "200",
            FontWeight::Ultralight => "100",
        };

        String::from(format!("font-weight: {}", weight))
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
        let css = format!(
            "font-size: {}rem;{};font-family: {}, {};",
            self.size,
            self.weight.css(),
            self.family,
            self.design
        );
        String::from(css)
    }
}
