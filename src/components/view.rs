use super::Component;

pub struct View<T>
    where T: Component
{
    child: T
}

impl<T> View<T>
    where T: Component
{
    pub fn new(child: T) -> View<T> {
        View {
            child
        }
    }
}

impl<T> Component for View<T>
    where T: Component
{
    fn html(&self) -> String {
        format!(r#"
            <html>
                <head>
                    <title>Mule</title>
                </head>
                <body>{}</body>
            </html>
        "#, self.child.html())
    }

    fn css(&self) -> String {
        String::from("")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        use std::io::prelude::*;
        use crate::components::text::Text;

        let bar = View::new(Text::new("thing")).render();

        let mut file = std::fs::File::create("foo.html").unwrap();
        file.write_all(bar.as_bytes()).unwrap();
    }
}