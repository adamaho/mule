use core::components::text::{FontStyle, Text};
use core::components::vstack::VStack;
use core::style::text::Color;

pub struct MyCustomComponent;

impl MyCustomComponent {
    pub fn new() -> VStack {
        VStack::new(vec![
            Box::new(
                Text::new("This is Fun I think")
                    .with_font(FontStyle::Heading1)
                    .with_color(Color::Green),
            ),
            Box::new(VStack::new(vec![
                Box::new(
                    Text::new("This is a paragraph")
                        .with_font(FontStyle::Heading3)
                        .with_color(Color::Purple),
                ),
                Box::new(
                    Text::new("Sweetie is awesome")
                        .with_font(FontStyle::Code)
                        .with_color(Color::Blue),
                ),
            ])),
        ])
    }
}
