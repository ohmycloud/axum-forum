use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};
use axum_messages::Messages;
use tower_sessions::Session;

use crate::{
    AppState,
    models::{CreatePostTemplate, HomeTemplate, Post, User},
};

pub async fn home_page() -> impl IntoResponse {
    "Post HomePage!"
}

pub fn posts_router() -> Router<AppState> {
    Router::new()
        .route("/post", get(posts))
        .route("/posts/create", get(create_post))
}

pub async fn create_post(messages: Messages, session: Session) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message.message))
        .collect::<Vec<String>>();
    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();

    let tmpl = CreatePostTemplate {
        title: "Create Post",
        messages,
        auth_user,
    };

    Html(tmpl.render().unwrap()).into_response()
}

pub async fn posts(
    session: Session,
    messages: Messages,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message.message))
        .collect::<Vec<String>>();
    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();
    let posts = Post::all(&state.pool).await.unwrap_or_else(|_| vec![]);
    let tmpl = HomeTemplate {
        title: "Posts Page",
        auth_user,
        messages,
        posts,
    };
    Html(tmpl.render().unwrap()).into_response()
}
