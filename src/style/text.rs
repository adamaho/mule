use super::CSS;

#[derive(Debug)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Purple
}

impl CSS for Color {
    fn css(&self) -> String {
        let color = match *self {
            Color::Red => "red",
            Color::Orange => "orange",
            Color::Yellow => "yellow",
            Color::Green => "green",
            Color::Blue => "blue",
            Color::Purple => "purple"
        };

        format!("color: {};", color)
    }
}

pub enum Alignment {
    Center,
    Left,
    Right
}

pub enum Decoration {
    Underline,
    Overline,
    LineThrough
}

pub enum Transform {
    Uppercase,
    Lowercase,
    Capitalize
}

#[derive(Debug)]
pub struct Text {
    pub color: Color
}

impl CSS for Text {
    fn css(&self) -> String {
        self.color.css()
    }
}
