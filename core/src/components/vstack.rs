use super::{Component, CSS};
use crate::html;
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
            },
        }
    }

    pub fn with_alignment(mut self, alignment: VStackHorizontalAlignment) -> VStack {
        self.style.horizontal_alignment = alignment;
        self
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
            ".{} {{ height: 100%; {} }} {}",
            self.class,
            self.style.horizontal_alignment.css(),
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
    horizontal_alignment: VStackHorizontalAlignment,
}
