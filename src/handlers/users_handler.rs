use askama::Template;
use axum::{
    Form, Router, debug_handler,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_messages::Messages;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Template)]
#[template(path = "../templates/pages/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    messages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterForm {
    pub name: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

pub async fn register_handler() -> impl IntoResponse {
    let tmpl = RegisterTemplate {
        title: "Register Page",
        messages: vec![],
    };

    Html(tmpl.render().unwrap())
}

pub async fn login_handler() -> impl IntoResponse {
    "Login page"
}

pub async fn register_form(Form(form): Form<RegisterForm>) -> Redirect {
    Redirect::to("/")
}

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_handler))
        .route("/register", get(register_handler))
        .route("/register", post(register_form))
}
