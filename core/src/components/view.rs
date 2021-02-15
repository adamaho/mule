use super::Component;
use crate::theme::Color;

pub struct View<T>
where
    T: Component,
{
    child: T,
    title: String,
    background: Color
}

impl<T> View<T>
where
    T: Component,
{
    pub fn new(child: T) -> View<T> {
        View {
            child,
            title: String::from("title"),
            background: Color::Gray50
        }
    }

    /// Change the document title of the page
    pub fn with_title(mut self, title: &str) -> View<T> {
        self.title = String::from(title);
        self
    }


    pub fn with_background(mut self, color: Color) -> View<T> {
        self.background = color;
        self
    }

    pub fn render(&self) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>{}</title>
                </head>
                <body>{}</body>
                <style>
                    html {{
                        margin: 0;
                        height: 100vh;
                        width: 100vw;
                    }}

                    body {{
                        height: 100%;
                        margin: 0;
                        background: {};
                    }}

                    * {{
                        box-sizing: border-box;
                    }}

                    {}
                </style>
            </html>
        "#,
            self.title,
            self.child.html(),
            self.background.hex(),
            self.child.css()
        )
    }
}
