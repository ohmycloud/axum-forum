use serde::{Deserialize, Serialize};
use validator::Validate;

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

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginForm {
    #[validate(email(message = "Invalid email"))]
    pub email: String,
    #[validate(length(min = 8, message = "Password must be more than 8 characters"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostForm {
    #[validate(length(min = 8, message = "Title must be at least 8 characters long"))]
    pub title: String,
    #[validate(length(min = 50, message = "Content must be at least 50 characters long"))]
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct PostUserForm {
    #[validate(length(min = 8, message = "Title must be at least 8 characters long"))]
    pub title: String,
    #[validate(length(min = 50, message = "Content must be at least 50 characters long"))]
    pub content: String,
    pub user_id: i32,
}
