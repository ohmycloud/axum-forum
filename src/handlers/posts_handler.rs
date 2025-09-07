use axum::{Router, response::IntoResponse, routing::get};

use crate::AppState;

pub async fn home_page() -> impl IntoResponse {
    "Post HomePage!"
}

pub fn posts_router() -> Router<AppState> {
    Router::new().route("/post", get(home_page))
}
