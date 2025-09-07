use axum::{Router, routing::get};

use crate::{
    AppState,
    handlers::{posts_handler::posts_router, users_handler::users_router},
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(users_router())
        .merge(posts_router())
        .route("/", get(|| async { "Hello!" }))
}
