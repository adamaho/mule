use core::components::text::{Text, TextAlignment, TextType};
use core::components::vstack::VStack;
use core::components::Component;

pub struct MyCustomComponent;

pub fn write_paragraphs() -> Box<VStack> {
    let mut paragraphs: Vec<Box<dyn Component>> = Vec::new();

    for _ in 0..10 {
        paragraphs.push(Box::new(
            Text::new("this is a paragraph").with_alignment(TextAlignment::Center),
        ));
    }

    Box::new(VStack::new(paragraphs))
}

impl MyCustomComponent {
    pub fn new() -> VStack {
        VStack::new(vec![
            Box::new(
                Text::new("Mule")
                    .with_text_type(TextType::Heading1)
                    .with_alignment(TextAlignment::Center),
            ),
            write_paragraphs(),
        ])
    }
}
