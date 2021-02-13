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
}

impl<T> Component for View<T>
where
    T: Component,
{
    fn html(&self) -> String {
        format!(
            r#"
            <html>
                <head>
                    <title>Mule</title>
                </head>
                <body>{}</body>
            </html>
        "#,
            self.child.html()
        )
    }

    fn css(&self) -> String {
        self.child.css()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        use crate::components::text::FontStyle;
        use crate::components::text::Text;
        use crate::style::text::Color;
        use std::io::prelude::*;

        let bar = View::new(
            Text::new("thing")
                .with_font(FontStyle::Heading1)
                .with_color(Color::Blue),
        )
        .render();

        let mut file = std::fs::File::create("foo.html").unwrap();
        file.write_all(bar.as_bytes()).unwrap();
    }
}
