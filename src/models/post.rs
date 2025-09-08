use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};

use crate::models::PostForm;

#[derive(Debug, Clone, Default, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    #[sqlx(skip)]
    pub user_name: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
}

impl Post {
    pub async fn create(pool: &PgPool, post: PostForm, auth_id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO posts (title, content, user_id)
            VALUES ($1, $2, $3)
            "#,
        )
        .bind(post.title)
        .bind(post.content)
        .bind(auth_id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn all(pool: &PgPool) -> anyhow::Result<Vec<Post>> {
        let posts = sqlx::query_as::<_, Post>(
            r#"
            SELECT users.name AS user_name, posts.*
            FROM posts
            LEFT JOIN users ON users.id = posts.user_id
            ORDER BY posts.created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(posts)
    }

    pub async fn find(pool: &PgPool, id: i32) -> anyhow::Result<Post> {
        let post = sqlx::query_as::<_, Post>(
            r#"
            SELECT posts.*, users.name AS user_name
            FROM posts
            JOIN users ON users.id = posts.user_id
            WHERE posts.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(pool)
        .await?;

        Ok(post)
    }

    pub async fn delete(pool: &PgPool, id: i32) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(pool: &PgPool, id: i32, post: PostForm) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            UPDATE posts
            SET title = $1, content = $2
            WHERE id = $3
            "#,
        )
        .bind(post.title)
        .bind(post.content)
        .bind(id)
        .execute(pool)
        .await?;

        Ok(())
    }
}
