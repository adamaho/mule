pub mod text;
pub mod view;
pub mod vstack;

pub trait Component {
    fn html(&self) -> String;
    fn css(&self) -> String;
}

#[macro_export]
macro_rules! html {
    ($tag:ident, $class:ident, $content:ident) => {
        format!("<{} class=\"{}\">{}</{}>", $tag, $class, $content, $tag)
    };
}
