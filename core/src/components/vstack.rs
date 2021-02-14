use super::Component;
use crate::html;
use crate::utils::make_class;

pub struct VStack {
    children: Vec<Box<dyn Component>>,
    class: String,
}

impl VStack {
    pub fn new(children: Vec<Box<dyn Component>>) -> VStack {
        VStack {
            children,
            class: make_class(),
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

        format!(".{} {{ height: 100%; }} {}", self.class, css)
    }
}
