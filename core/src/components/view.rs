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
