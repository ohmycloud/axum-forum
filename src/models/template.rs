use askama::Template;

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
