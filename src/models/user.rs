use chrono::{DateTime, Local};
use password_auth::{generate_hash, verify_password};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

use crate::models::{LoginForm, RegisterForm};

#[derive(Debug, Serialize, Deserialize, Clone, Default, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Local>,
}

impl User {
    pub async fn login(pool: &PgPool, form: LoginForm) -> anyhow::Result<Self> {
        let user: User = sqlx::query_as(
            r#"
            SELECT * FROM users WHERE email = $1
            "#,
        )
        .bind(&form.email)
        .fetch_one(pool)
        .await?;

        let input_password = user.password.clone();
        tokio::task::spawn_blocking(move || verify_password(&form.password, &input_password))
            .await?;

        Ok(user)
    }

    pub async fn register(pool: &PgPool, form: RegisterForm) -> anyhow::Result<Self> {
        let hashed_password =
            tokio::task::spawn_blocking(move || generate_hash(&form.password)).await?;
        let user: User = sqlx::query_as(
            r#"
            INSERT INTO users (name, email, password)
            VALUES ($1, $2, $3) returning *
            "#,
        )
        .bind(&form.name)
        .bind(&form.email)
        .bind(hashed_password)
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn email_exists(pool: &PgPool, email: &str) -> anyhow::Result<bool> {
        let count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*) FROM users WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_one(pool)
        .await?;

        Ok(count > 0)
    }
}
