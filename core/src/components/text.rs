use std::default::Default;
use std::fmt;

use super::{Component, CSS};
use crate::theme::Color;
use crate::html;
use crate::utils;

/// A component that displays one or more lines of text.
///
///
/// ## Example
/// ```
/// Text::new("This is an example");
/// ```
pub struct Text {
    child: String,
    class: String,
    html_tag: HTMLTextTag,
    style: Style,
}

impl Text {
    /// Create a new `Text` element.
    ///
    /// ```
    /// Text::new("This is a text element")
    /// ```
    pub fn new(content: &str) -> Text {
        Text {
            child: content.to_string(),
            html_tag: HTMLTextTag::P,
            class: utils::make_class(),
            style: Style {
                font: Font {
                    size: 1.0,
                    weight: FontWeight::Regular,
                    family: String::from("Lato"),
                    design: String::from("sans-serif"),
                },
                color: Color::Gray50,
                alignment: TextAlignment::Leading,
                decoration: TextDecoration::None,
                transform: TextTransform::None,
            },
        }
    }

    /// Modify the semantics and styling of the text element using `with_text_type`. For example,
    /// If we wanted to change the semantics to match that of a heading we can do the following:
    ///
    /// ```
    /// Text::new("Text as Heading").with_text_type(TextStyle::Heading1)
    /// ```
    pub fn with_text_type(mut self, text_type: TextType) -> Text {
        match text_type {
            TextType::Heading1 => {
                self.html_tag = HTMLTextTag::H1;
                self.style.font.size = 3.0;
                self.style.font.weight = FontWeight::Heavy;
                self
            }
            TextType::Heading2 => {
                self.html_tag = HTMLTextTag::H2;
                self.style.font.size = 2.25;
                self.style.font.weight = FontWeight::Bold;
                self
            }
            TextType::Heading3 => {
                self.html_tag = HTMLTextTag::H3;
                self.style.font.size = 1.875;
                self.style.font.weight = FontWeight::Bold;
                self
            }
            TextType::Heading4 => {
                self.html_tag = HTMLTextTag::H4;
                self.style.font.size = 1.5;
                self.style.font.weight = FontWeight::Bold;
                self
            }
            TextType::Heading5 => {
                self.html_tag = HTMLTextTag::H5;
                self.style.font.size = 1.25;
                self.style.font.weight = FontWeight::Semibold;
                self
            }
            TextType::Heading6 => {
                self.html_tag = HTMLTextTag::H6;
                self.style.font.size = 1.125;
                self.style.font.weight = FontWeight::Semibold;
                self
            }
            TextType::Paragraph => {
                self.html_tag = HTMLTextTag::P;
                self.style.font.size = 1.0;
                self.style.font.weight = FontWeight::Regular;
                self
            }
            TextType::Code => {
                self.html_tag = HTMLTextTag::Code;
                self.style.font.size = 1.0;
                self.style.font.weight = FontWeight::Regular;
                self.style.font.family = String::from("monospace");
                self
            }
        }
    }

    pub fn with_color(mut self, color: Color) -> Text {
        self.style.color = color;
        self
    }

    pub fn with_alignment(mut self, alignment: TextAlignment) -> Text {
        self.style.alignment = alignment;
        self
    }
}

impl Component for Text {
    /// Render the HTMl output of a Text element
    fn html(&self) -> String {
        let tag = &self.html_tag;
        let class = &self.class;
        let child = &self.child;

        html!(tag, class, child)
    }

    /// Render the CSS output of a Text element
    fn css(&self) -> String {
        format!(
            ".{} {{margin:0;color:{};{}{}{}{}}}",
            self.class,
            self.style.color.hex(),
            self.style.font.css(),
            self.style.alignment.css(),
            self.style.transform.css(),
            self.style.decoration.css()
        )
    }
}

/// The style of the
struct Style {
    font: Font,
    color: Color,
    alignment: TextAlignment,
    decoration: TextDecoration,
    transform: TextTransform,
}

/// The semantic text type for the text element
pub enum TextType {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Paragraph,
    Code,
}

/// The HTML tag to describe the text element
enum HTMLTextTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Code,
}

/// Convert the enum to an HTML tag string
impl fmt::Display for HTMLTextTag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let tag = match *self {
            HTMLTextTag::H1 => String::from("h1"),
            HTMLTextTag::H2 => String::from("h2"),
            HTMLTextTag::H3 => String::from("h3"),
            HTMLTextTag::H4 => String::from("h4"),
            HTMLTextTag::H5 => String::from("h5"),
            HTMLTextTag::H6 => String::from("h6"),
            HTMLTextTag::P => String::from("p"),
            HTMLTextTag::Code => String::from("code"),
        };
        write!(f, "{}", tag)
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
