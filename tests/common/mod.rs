use axum::{
    Router,
    body::{to_bytes, Body},
    http::Response,
};
use std::sync::Once;
use sqlx::{
    PgPool,
    postgres::PgPoolOptions,
};

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").ok();
    });
}

pub async fn create_test_app() -> (Router, PgPool) {
    init();

    let pool = create_test_db_pool().await;
    let app = football_group_manager::create_app_with_pool(pool.clone());
    clean_db(&pool).await;

    (app, pool)
}

async fn create_test_db_pool() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let max_connections = std::env::var("MAX_DB_CONNECTIONS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(10);

    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

pub async fn clean_db(pool: &PgPool) {
    sqlx::query("TRUNCATE TABLE users RESTART IDENTITY CASCADE")
        .execute(pool)
        .await
        .expect("Failed to clean database");
}

pub async fn parse_response_body<T: serde::de::DeserializeOwned>(
    response: Response<Body>
) -> T {
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    serde_json::from_str(&body).expect("Failed to parse response body")
}