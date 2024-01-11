use crate::{
    app_state::AppState,
    routes::{hello_world::hello_world, users::create_user::create_user},
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/api/v1/users", post(create_user))
        .with_state(app_state)
}
