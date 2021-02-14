pub mod text;
pub mod view;
pub mod vstack;

/// Implemented to provide a struct or enum the ability to render custom html or css.
pub trait Component {
    /// html
    fn html(&self) -> String;

    /// css
    fn css(&self) -> String;
}

/// Trait that is required to convert to css
pub trait CSS {
    fn css(&self) -> String;
}

#[macro_export]
macro_rules! html {
    ($tag:ident, $class:ident, $content:ident) => {
        format!("<{} class=\"{}\">{}</{}>", $tag, $class, $content, $tag)
    };
}
