# Mule (Definitely a Work in Progress)

The night I started this project I was on the couch drinking a Moscow Mule. Couldn't think of a better name, so for now its called mule. TLDR; Naming is hard.

Truthfully, not sure if it is worth the work because HTML, CSS and Javascript do this a lot better than rust. This is more of a fun thought experiment to see if there would be any benefits to doing this in rust. So far the only benefit I see is a developer doesnt need to know HTML or CSS to create a static blog site in rust.

### Example
Use any rust web framework like actix, rocket, warp or tide and return a `text/html` string in order. Below is the example of what it looks like to create a simple html page using Mule and Warp as the web framework.

```rust
use warp::Filter;

pub mod component;

use core::components::view::View;
use core::components::text::{Text, TextType, TextColor};
use core::components::vstack::VStack;

#[tokio::main]
async fn main() {
    let route =
        warp::any().map(move || warp::reply::html(View::new(
            VStack::new(vec![
                Box::new(Text::new("My Doggie Site").with_text_type(TextType::Heading1)),
                Box::new(Text::new("My Dog is the best dog in the world").with_color(TextColor::Purple))
            ])
        )
        .with_title("My Website Title")
        .render()));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
```

This creates the following HTML and CSS:

```html
<!DOCTYPE html>
<html>
    <head>
        <title>My Website Title</title>
    </head>
    <body><div class="mule-vlsqsy"><h1 class="mule-yetcse">My Doggie Site</h1>
<p class="mule-uqsgpz">My Dog is the best dog in the world</p>
</div></body>
    <style>
        html {
            margin: 0;
            height: 100vh;
            width: 100vw;
        }

        body {
            height: 100%;
            margin: 0;
        }

        .mule-vlsqsy { height: 100%; } .mule-yetcse {margin:0;font-size: 3rem;font-weight: 800;font-family: Lato, sans-serif;color: black;text-align: left;text-decoration: none;text-decoration: none;}
        .mule-uqsgpz {margin:0;font-size: 1rem;font-weight: 400;font-family: Lato, sans-serif;color: purple;text-align: left;text-decoration: none;text-decoration: none;}
    </style>
</html>
```