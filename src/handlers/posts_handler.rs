use askama::Template;
use axum::{
    Form, Router,
    extract::{Path, State},
    response::{Html, IntoResponse, Redirect},
    routing::{delete, get, post},
};
use axum_messages::Messages;
use tower_sessions::Session;
use validator::Validate;

use crate::{AppState, models::*, utils::validation_errors};

pub async fn home_page() -> impl IntoResponse {
    "Post HomePage!"
}

pub fn posts_router() -> Router<AppState> {
    Router::new()
        .route("/", get(posts))
        .route("/posts/create", get(create_post))
        .route("/posts", post(save_post))
        .route("/posts/{id}/update", post(update_post))
        .route("/posts/{id}/delete", delete(delete_post))
        .route("/posts/{id}/edit", get(edit_post))
        .route("/posts/{id}", get(show_post))
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

pub async fn show_post(
    session: Session,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();
    let post = Post::find(&state.pool, id).await;
    if let Err(_) = post {
        return Redirect::to("/").into_response();
    }

    let tmpl = ShowPostTemplate {
        title: "Show Post",
        auth_user,
        post: post.unwrap(),
    };

    Html(tmpl.render().unwrap()).into_response()
}

pub async fn edit_post(
    messages: Messages,
    session: Session,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message.message))
        .collect::<Vec<String>>();
    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();
    let post = match Post::find(&state.pool, id).await {
        Ok(post) => post,
        Err(_) => return Redirect::to("/").into_response(),
    };

    let tmpl = EditPostTemplate {
        title: "Edit Post",
        messages,
        auth_user,
        post,
    };

    Html(tmpl.render().unwrap()).into_response()
}

pub async fn update_post(
    messages: Messages,
    session: Session,
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Form(form): Form<PostForm>,
) -> impl IntoResponse {
    if let Err(errors) = form.validate() {
        let error_messages = validation_errors(errors);
        let mut messages = messages;
        for error in error_messages {
            messages = messages.error(error);
        }
        return Redirect::to("/posts/create").into_response();
    }

    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();
    let post = match Post::find(&state.pool, id).await {
        Ok(post) => post,
        Err(_) => return Redirect::to("/").into_response(),
    };

    if post.user_id != auth_user.id {
        return Redirect::to("/").into_response();
    }

    if let Err(_) = Post::update(&state.pool, id, form).await {
        messages.error("Failed to update post");
        return Redirect::to("/posts/create").into_response();
    } else {
        messages.success("Post updated successfully");
        return Redirect::to("/").into_response();
    }
}

pub async fn delete_post(
    messages: Messages,
    session: Session,
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> impl IntoResponse {
    let auth_user: User = session.get("auth_user").await.unwrap().unwrap();
    let post = match Post::find(&state.pool, id).await {
        Ok(post) => post,
        Err(_) => return Redirect::to("/").into_response(),
    };

    if post.user_id != auth_user.id {
        return Redirect::to("/").into_response();
    }

    if let Err(_) = Post::delete(&state.pool, id).await {
        messages.error("Failed to delete post");
        return Redirect::to("/").into_response();
    } else {
        messages.success("Post deleted successfully");
        return Redirect::to("/").into_response();
    }
}
