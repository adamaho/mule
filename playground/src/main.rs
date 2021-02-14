use warp::Filter;

pub mod component;

use core::components::view::View;

use crate::component::MyCustomComponent;

#[tokio::main]
async fn main() {
    let route =
        warp::any().map(move || warp::reply::html(View::new(MyCustomComponent::new()).render()));

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
