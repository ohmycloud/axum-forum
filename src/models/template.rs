use askama::Template;

use crate::models::{Post, User};

#[derive(Debug, Template)]
#[template(path = "../templates/pages/register.html")]
pub struct RegisterTemplate<'a> {
    pub title: &'a str,
    pub messages: Vec<String>,
}

#[derive(Debug, Template)]
#[template(path = "../templates/pages/login.html")]
pub struct LoginTemplate<'a> {
    pub title: &'a str,
    pub messages: Vec<String>,
}

#[derive(Debug, Template)]
#[template(path = "../templates/pages/home.html")]
pub struct HomeTemplate<'a> {
    pub title: &'a str,
    pub auth_user: User,
    pub messages: Vec<String>,
    pub posts: Vec<Post>,
}

#[derive(Debug, Template)]
#[template(path = "../templates/pages/create-post.html")]
pub struct CreatePostTemplate<'a> {
    pub title: &'a str,
    pub messages: Vec<String>,
    pub auth_user: User,
}
