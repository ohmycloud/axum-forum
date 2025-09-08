use askama::Template;
use axum::{
    Form, Router,
    extract::State,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_messages::Messages;
use tower_sessions::Session;
use validator::Validate;

use crate::{
    AppState,
    models::{CreatePostTemplate, HomeTemplate, Post, PostForm, User},
    utils::validation_errors,
};

pub async fn home_page() -> impl IntoResponse {
    "Post HomePage!"
}

pub fn posts_router() -> Router<AppState> {
    Router::new()
        .route("/post", get(posts))
        .route("/posts/create", get(create_post))
        .route("/posts", post(save_post))
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

pub async fn save_post(
    messages: Messages,
    session: Session,
    State(state): State<AppState>,
    Form(form): Form<PostForm>,
) -> impl IntoResponse {
    if let Err(errors) = form.validate() {
        let error_messages = validation_errors(errors);
        let mut messages = messages;
        for error in error_messages {
            messages = messages.error(error);
        }
        Redirect::to("/posts/create")
    } else {
        let auth_user: User = session.get("auth_user").await.unwrap().unwrap();
        if let Err(error) = Post::create(&state.pool, form, auth_user.id).await {
            messages.error("Failed to create post");
            Redirect::to("/posts/create")
        } else {
            messages.success("Post created successfully");
            Redirect::to("/")
        }
    }
}
