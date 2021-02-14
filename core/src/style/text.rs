use super::CSS;

pub enum TextColor {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple,
    Black,
    White,
}

impl CSS for TextColor {
    fn css(&self) -> String {
        let color = match *self {
            TextColor::Red => "red",
            TextColor::Orange => "orange",
            TextColor::Yellow => "yellow",
            TextColor::Green => "green",
            TextColor::Blue => "blue",
            TextColor::Purple => "purple",
            TextColor::Black => "black",
            TextColor::White => "white",
        };

        format!("color: {};", color)
    }
}

pub enum TextAlignment {
    Center,
    Leading,
    Trailing,
}

impl CSS for TextAlignment {
    fn css(&self) -> String {
        let alignment = match *self {
            TextAlignment::Center => "center",
            TextAlignment::Leading => "left",
            TextAlignment::Trailing => "right",
        };

        format!("text-align: {};", alignment)
    }
}

pub enum TextDecoration {
    Underline,
    Overline,
    LineThrough,
    None,
}

impl CSS for TextDecoration {
    fn css(&self) -> String {
        let decoration = match *self {
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::LineThrough => "line-through",
            TextDecoration::None => "none",
        };

        format!("text-decoration: {};", decoration)
    }
}

pub enum TextTransform {
    Uppercase,
    Lowercase,
    Capitalize,
    None,
}

impl CSS for TextTransform {
    fn css(&self) -> String {
        let transform = match *self {
            TextTransform::Uppercase => "uppercase",
            TextTransform::Lowercase => "lowercase",
            TextTransform::Capitalize => "capitalize",
            TextTransform::None => "none",
        };

        format!("text-decoration: {};", transform)
    }
}
