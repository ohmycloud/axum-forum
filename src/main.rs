#![allow(unused)]

mod handlers;
mod router;

use axum_messages::MessagesManagerLayer;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::{env, net::SocketAddr};
use tower_http::services::ServeDir;
use tower_sessions::{Expiry, SessionManagerLayer, cookie::time::Duration};
use tower_sessions_sqlx_store::PostgresStore;

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
    let app_state = AppState { pool: pool.clone() };
    let serve_dir = ServeDir::new("assets").not_found_service(ServeDir::new("assets/index.html"));
    let session_store = PostgresStore::new(pool.clone());
    session_store.migrate().await.unwrap();

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(60 * 60 * 24)));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("Listening on: http://{}", addr);

    let app = router::routes()
        .layer(MessagesManagerLayer)
        .layer(session_layer)
        .nest_service("/assets", serve_dir)
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
