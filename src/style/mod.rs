pub mod font;

// trait that is required to convert to css
pub trait CSS {
    fn css(&self) -> String;
}
