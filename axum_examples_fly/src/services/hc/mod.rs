mod hc;
mod hello_world;

use axum::{routing::get, Router};
use hello_world::*;
use hc::*;

pub fn get_routes() -> Router {
    Router::new()
        .route("/", get(get_hello_world))
        .route("/hc", get(check_hc))
}