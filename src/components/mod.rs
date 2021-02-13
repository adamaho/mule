pub mod text;

pub trait Component {
    fn html(&self) -> String;
    fn css(&self) -> String;

    fn render(&self) {
        let html = self.html();
        let css = self.css();

        println!("{:#?}", html);
        println!("{:#?}", css);
    }
}
