pub mod app;
pub mod routes;
pub mod dto;

use app::{router::create_router, state::AppState};
use axum::Router;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;

pub fn create_app_with_pool(pool: PgPool) -> Router {
	create_router(AppState { db: pool })
}

pub fn create_app() -> Router {
	let database_url = env::var("DATABASE_URL").unwrap();

	create_app_lazy(
        database_url,
        env::var("MAX_DB_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10),
    )
}

pub fn create_app_lazy(database_url: String, max_connections: u32) -> Router {
	let pool = PgPoolOptions::new()
		.max_connections(max_connections)
		.connect_lazy(&database_url)
		.expect("failed to create lazy database pool");

	create_app_with_pool(pool)
}