use super::Component;

pub struct VStack {
    pub children: Vec<Box<dyn Component>>,
}

impl VStack {
    pub fn new(children: Vec<Box<dyn Component>>) -> VStack {
        VStack { children }
    }
}

impl Component for VStack {
    fn html(&self) -> String {
        let mut html = String::new();

        for child in self.children.iter() {
            html.push_str(&format!("{}\n", child.html()));
        }

        html
    }

    fn css(&self) -> String {
        let mut css = String::new();

        for child in self.children.iter() {
            css.push_str(&format!("{}\n", child.css()));
        }

        css
    }
}
