use warp::Filter;

pub mod component;

use core::components::text::{Text, TextType};
use core::components::view::View;
use core::components::vstack::{VStack};
use core::theme::{Color, Edges};

#[tokio::main]
async fn main() {
    let route = warp::any().map(move || {
        warp::reply::html(
            View::new(
                VStack::new(vec![
                    Box::new(Text::new("My Doggie Site").with_text_type(TextType::Heading1)),
                    Box::new(
                        Text::new("My Dog is the best dog in the world")
                    ),
                ])
                .with_padding(Edges::Vertical, 1.5)
                .with_padding(Edges::Horizontal, 3.5)
            )
            .with_title("My Website Title")
            .with_background(Color::Gray900)
            .render(),
        )
    });

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
