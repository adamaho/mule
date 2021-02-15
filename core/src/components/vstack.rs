use super::{Component, CSS};

use crate::html;
use crate::theme::{Color, Padding, Edges};
use crate::utils::make_class;

pub struct VStack {
    children: Vec<Box<dyn Component>>,
    class: String,
    style: Style,
}

impl VStack {
    pub fn new(children: Vec<Box<dyn Component>>) -> VStack {
        VStack {
            children,
            class: make_class(),
            style: Style {
                horizontal_alignment: VStackHorizontalAlignment::Leading,
                background: Color::Gray50,
                padding: Padding {
                    top: 0.0,
                    right: 0.0,
                    bottom: 0.0,
                    left: 0.0
                }
            },
        }
    }

    pub fn with_alignment(mut self, alignment: VStackHorizontalAlignment) -> VStack {
        self.style.horizontal_alignment = alignment;
        self
    }

    pub fn with_background(mut self, color: Color) -> VStack {
        self.style.background = color;
        self
    }

    /// TODO: Turn this into a derive macro for better reusability
    pub fn with_padding(mut self, edges: Edges, amount: f32) -> VStack {
        match edges {
            Edges::Top => {
                self.style.padding.top = amount;
                self
            },
            Edges::Right => {
                self.style.padding.right = amount;
                self
            },
            Edges::Bottom => {
                self.style.padding.bottom = amount;
                self
            },
            Edges::Left => {
                self.style.padding.left = amount;
                self
            },
            Edges::All => {
                self.style.padding.top = amount;
                self.style.padding.right = amount;
                self.style.padding.bottom = amount;
                self.style.padding.left = amount;
                self
            },
            Edges::Horizontal => {
                self.style.padding.right = amount;
                self.style.padding.left = amount;
                self
            },
            Edges::Vertical => {
                self.style.padding.top = amount;
                self.style.padding.bottom = amount;
                self
            },
        }
    }
}

impl Component for VStack {
    fn html(&self) -> String {
        let mut content = String::new();

        for child in self.children.iter() {
            content.push_str(&format!("{}\n", child.html()));
        }

        let tag = "div";
        let class = &self.class;

        html!(tag, class, content)
    }

    fn css(&self) -> String {
        let mut css = String::new();

        for child in self.children.iter() {
            css.push_str(&format!("{}\n", child.css()));
        }

        format!(
            ".{} {{ height: 100%; {}{} }} {}",
            self.class,
            self.style.horizontal_alignment.css(),
            self.style.padding.css(),
            css
        )
    }
}

pub enum VStackHorizontalAlignment {
    Leading,
    Center,
    Trailing,
}

impl CSS for VStackHorizontalAlignment {
    fn css(&self) -> String {
        let horizontal_alignment = match *self {
            VStackHorizontalAlignment::Leading => "flex-start",
            VStackHorizontalAlignment::Center => "center",
            VStackHorizontalAlignment::Trailing => "flex-end",
        };

        format!(
            r#"
            display: flex;
            flex-direction: column;
            align-items: {};
        "#,
            horizontal_alignment
        )
    }
}

struct Style {
    background: Color,
    horizontal_alignment: VStackHorizontalAlignment,
    padding: Padding
}
