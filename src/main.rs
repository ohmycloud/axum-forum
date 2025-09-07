#![allow(unused)]

mod handlers;
mod router;

use dotenvy::dotenv;
use sqlx::PgPool;
use std::env;

#[derive(Debug, Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database pool");
    let app_state = AppState { pool };

    let addr = "127.0.0.1:8080";
    println!("Listening on: http://{}", addr);
    let app = router::routes().with_state(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
