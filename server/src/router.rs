use crate::routes::hello_world::hello_world;
use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/", get(hello_world))
}
