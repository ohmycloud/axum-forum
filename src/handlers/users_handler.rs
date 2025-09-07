use askama::Template;
use axum::{
    Form, Router, debug_handler,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_messages::Messages;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::AppState;

#[derive(Debug, Template)]
#[template(path = "../templates/pages/register.html")]
struct RegisterTemplate<'a> {
    title: &'a str,
    messages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterForm {
    #[validate(length(
        min = 4,
        max = 50,
        message = "Name must be between 4 and 50 characters"
    ))]
    pub name: String,
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be more than 8 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Passwords do not match"))]
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
    if let Err(errors) = form.validate() {
        Redirect::to("/register")
    } else {
        Redirect::to("/")
    }
}

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_handler))
        .route("/register", get(register_handler))
        .route("/register", post(register_form))
}
