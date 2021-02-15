use warp::Filter;

pub mod component;

use core::components::text::{Text, TextColor, TextType};
use core::components::view::View;
use core::components::vstack::{VStack};

#[tokio::main]
async fn main() {
    let route = warp::any().map(move || {
        warp::reply::html(
            View::new(
                VStack::new(vec![
                    Box::new(Text::new("My Doggie Site").with_text_type(TextType::Heading1)),
                    Box::new(
                        Text::new("My Dog is the best dog in the world")
                            .with_color(TextColor::Purple),
                    ),
                ]),
            )
            .with_title("My Website Title")
            .render(),
        )
    });

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
