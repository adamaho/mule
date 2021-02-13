pub mod text;

pub trait Component {
    fn html(&self) -> String;
    fn css(&self) -> String;

    fn render(&self) -> String {
        let html = self.html();
        let css = self.css();

        format!(r#"
            {}
            <style>
            {}
            </style>
        "#, html, css)
    }
}

#[macro_export]
macro_rules! html {
    ($tag:ident, $class:ident, $content:ident) => {
        format!("<{} class=\"{}\">{}</{}>", $tag, $class, $content, $tag)
    };
}
