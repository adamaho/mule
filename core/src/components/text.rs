use std::fmt;

use crate::html;

use crate::style::font::{Font, FontWeight};
use crate::style::text::{Color, Text as TextStyle};
use crate::style::CSS;
use crate::utils;

use super::Component;

#[derive(Debug)]
pub enum FontStyle {
    Heading1,
    Heading2,
    Heading3,
    Heading4,
    Heading5,
    Heading6,
    Paragraph,
    Code,
}

#[derive(Debug)]
pub enum HTMLTextTag {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    P,
    Code,
}

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

#[derive(Debug)]
pub struct Text {
    child: String,
    class: String,
    html_tag: HTMLTextTag,
    font: Font,
    text_style: TextStyle,
}

impl Text {
    pub fn new(content: &str) -> Text {
        Text {
            child: content.to_string(),
            html_tag: HTMLTextTag::P,
            class: utils::make_class(),
            font: Font {
                size: 1.0,
                weight: FontWeight::Regular,
                family: String::from("Lato"),
                design: String::from("sans-serif"),
            },
            text_style: TextStyle { color: Color::Red },
        }
    }

    pub fn with_font(mut self, font_style: FontStyle) -> Text {
        match font_style {
            FontStyle::Heading1 => {
                self.html_tag = HTMLTextTag::H1;
                self.font.size = 3.0;
                self.font.weight = FontWeight::Heavy;
                self
            }
            FontStyle::Heading2 => {
                self.html_tag = HTMLTextTag::H2;
                self.font.size = 2.25;
                self.font.weight = FontWeight::Bold;
                self
            }
            FontStyle::Heading3 => {
                self.html_tag = HTMLTextTag::H3;
                self.font.size = 1.875;
                self.font.weight = FontWeight::Bold;
                self
            }
            FontStyle::Heading4 => {
                self.html_tag = HTMLTextTag::H4;
                self.font.size = 1.5;
                self.font.weight = FontWeight::Bold;
                self
            }
            FontStyle::Heading5 => {
                self.html_tag = HTMLTextTag::H5;
                self.font.size = 1.25;
                self.font.weight = FontWeight::Semibold;
                self
            }
            FontStyle::Heading6 => {
                self.html_tag = HTMLTextTag::H6;
                self.font.size = 1.125;
                self.font.weight = FontWeight::Semibold;
                self
            }
            FontStyle::Paragraph => {
                self.html_tag = HTMLTextTag::P;
                self.font.size = 1.0;
                self.font.weight = FontWeight::Regular;
                self
            }
            FontStyle::Code => {
                self.html_tag = HTMLTextTag::Code;
                self.font.size = 1.0;
                self.font.weight = FontWeight::Regular;
                self.font.family = String::from("monospace");
                self
            }
        }
    }

    pub fn with_color(mut self, color: Color) -> Text {
        self.text_style.color = color;
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
            ".{} {{{}{}}}",
            self.class,
            self.font.css(),
            self.text_style.css()
        )
    }
}

// text
// alignment
// color
// padding
