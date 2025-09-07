use askama::Template;
use axum::{
    Form, Router, debug_handler,
    response::{Html, IntoResponse, Redirect},
    routing::{get, post},
};
use axum_messages::Messages;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    AppState,
    models::{LoginForm, LoginTemplate, RegisterForm, RegisterTemplate},
    utils::validation_errors,
};

pub async fn register_handler(messages: Messages) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message.message))
        .collect::<Vec<_>>();

    let tmpl = RegisterTemplate {
        title: "Register Page",
        messages,
    };

    Html(tmpl.render().unwrap())
}

pub async fn login_handler(messages: Messages) -> impl IntoResponse {
    let messages = messages
        .into_iter()
        .map(|message| format!("{}: {}", message.level, message.message))
        .collect::<Vec<_>>();

    let tmpl = LoginTemplate {
        title: "Login",
        messages,
    };

    Html(tmpl.render().unwrap())
}

pub async fn register_form(messages: Messages, Form(form): Form<RegisterForm>) -> Redirect {
    // Validate the upcoming data
    if let Err(errors) = form.validate() {
        let error_messages = validation_errors(errors);
        let mut messages = messages;

        for error in error_messages {
            messages = messages.error(error)
        }

        Redirect::to("/register")
    } else {
        Redirect::to("/")
    }
}

pub async fn login_form(messages: Messages, Form(form): Form<LoginForm>) -> Redirect {
    // Validate the upcoming data
    if let Err(errors) = form.validate() {
        let error_messages = validation_errors(errors);
        let mut messages = messages;

        for error in error_messages {
            messages = messages.error(error)
        }

        Redirect::to("/login")
    } else {
        Redirect::to("/")
    }
}

pub fn users_router() -> Router<AppState> {
    Router::new()
        .route("/login", get(login_handler).post(login_form))
        .route("/register", get(register_handler).post(register_form))
}
