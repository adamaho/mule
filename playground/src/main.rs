use warp::Filter;

use core::components::text::FontStyle;
use core::components::text::Text;
use core::components::view::View;
use core::components::vstack::VStack;
use core::style::text::Color;

#[tokio::main]
async fn main() {
    let route = warp::any().map(move || {
        warp::reply::html(
            View::new(VStack::new(vec![
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
                    Box::new(
                        Text::new("This is a paragraph")
                            .with_font(FontStyle::Code)
                            .with_color(Color::Blue),
                    ),
                ])),
            ]))
            .render(),
        )
    });

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
