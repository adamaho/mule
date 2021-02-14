use super::Component;

pub struct View<T>
where
    T: Component,
{
    child: T,
    title: String
}

impl<T> View<T>
where
    T: Component,
{
    pub fn new(child: T) -> View<T> {
        View { child, title: String::from("title") }
    }

    /// Change the document title of the page
    pub fn with_title(mut self, title: &str) -> View<T> {
        self.title = String::from(title);
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
                    }}
                    {}
                </style>
            </html>
        "#,
            self.title,
            self.child.html(),
            self.child.css()
        )
    }
}
