use std::fmt;

use crate::html;

use crate::style::font::{Font, FontWeight};
use crate::style::text::{TextAlignment, TextColor, TextDecoration, TextTransform};
use crate::style::CSS;
use crate::utils;

use super::Component;

/// A component that displays one or more lines of text.
///
///
/// ## Example
/// ```
/// Text::new("This is an example");
/// ```
///
///
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
                color: TextColor::Black,
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
    ///
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

    /// Modify the color of the text based on the default color palette. 
    /// 
    /// ```
    /// Text::new("Text with Color").with_color(TextColor::Red)
    /// ```
    ///
    pub fn with_color(mut self, color: TextColor) -> Text {
        self.style.color = color;
        self
    }

    /// Modify the alignment of the text
    ///
    /// ```
    /// Text::new("Text with Color").with_alignment(TextAlignment::Center)
    /// ```
    ///
    pub fn with_alignment(mut self, alignment: TextAlignment) -> Text {
        self.style.alignment = alignment;
        self
    }
}

impl Component for Text {
    fn html(&self) -> String {
        let tag = &self.html_tag;
        let class = &self.class;
        let child = &self.child;

        html!(tag, class, child)
    }

    fn css(&self) -> String {
        format!(
            ".{} {{{}{}{}{}{}}}",
            self.class,
            self.style.font.css(),
            self.style.color.css(),
            self.style.alignment.css(),
            self.style.transform.css(),
            self.style.decoration.css()
        )
    }
}

struct Style {
    font: Font,
    color: TextColor,
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
