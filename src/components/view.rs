use super::Component;

pub struct View<T>
where
    T: Component,
{
    child: T,
}

impl<T> View<T>
where
    T: Component,
{
    pub fn new(child: T) -> View<T> {
        View { child }
    }

    pub fn render(&self) -> String {
        format!(
            r#"
            <html>
                <head>
                    <title>Mule</title>
                </head>
                <body>{}</body>
                <style>{}</style>
            </html>
        "#,
            self.child.html(),
            self.child.css()
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        use crate::components::text::FontStyle;
        use crate::components::text::Text;
        use crate::components::vstack::VStack;
        use crate::style::text::Color;
        use std::io::prelude::*;

        let page = View::new(VStack::new(vec![
            Box::new(
                Text::new("This is Fun I think")
                    .with_font(FontStyle::Heading1)
                    .with_color(Color::Green),
            ),
            Box::new(VStack::new(vec![
                Box::new(
                    Text::new("This is a paragraph")
                        .with_font(FontStyle::Heading3)
                        .with_color(Color::Red),
                ),
                Box::new(Text::new("This is a paragraph").with_font(FontStyle::Code).with_color(Color::Blue)),
            ])),
        ]))
        .render();

        let mut file = std::fs::File::create("foo.html").unwrap();
        file.write_all(page.as_bytes()).unwrap();
    }
}
