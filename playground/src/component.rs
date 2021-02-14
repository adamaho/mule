use core::components::text::{FontStyle, Text};
use core::components::vstack::VStack;
use core::style::text::{TextColor, TextAlignment};

pub struct MyCustomComponent;

impl MyCustomComponent {
    pub fn new() -> VStack {
        VStack::new(vec![
            Box::new(
                Text::new("This is Fun I think")
                    .with_font(FontStyle::Heading1)
                    .with_color(TextColor::Green)
                    .with_alignment(TextAlignment::Center),
            ),
            Box::new(VStack::new(vec![
                Box::new(
                    Text::new("This is a paragraph")
                        .with_font(FontStyle::Heading3)
                        .with_color(TextColor::Purple)
                        .with_alignment(TextAlignment::Trailing),
                ),
                Box::new(
                    Text::new("Sweetie is awesome")
                        .with_font(FontStyle::Code)
                        .with_color(TextColor::Blue),
                ),
            ])),
        ])
    }
}
