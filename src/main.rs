#[macro_use]
extern crate diesel;

mod config;
mod error;
mod globals;
mod models;
mod schema;

use warp::{http::Response, Filter};

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String).map(|name| format!("Hello, {}!", name));
    let not_found = warp::any().map(|| Response::builder().status(404).body("Not Found").unwrap());

    let routes = hello.or(not_found);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
