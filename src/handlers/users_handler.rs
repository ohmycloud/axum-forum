use askama::Template;
use axum::{
    Router,
    response::{Html, IntoResponse},
    routing::get,
};

use crate::AppState;

#[derive(Debug, Template)]
#[template(path = "../templates/partials/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
}

pub async fn register_handler() -> impl IntoResponse {
    let tmpl = RegisterTemplate { title: "Register" };
    Html(tmpl.render().unwrap())
}

pub async fn login_handler() -> impl IntoResponse {
    "Login page"
}

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_handler))
        .route("/register", get(register_handler))
}
