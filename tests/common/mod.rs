use axum::{
    Router,
    body::{to_bytes, Body},
    http::Response,
};
use std::sync::Once;

pub mod defaut_data;

static INIT: Once = Once::new();

fn init() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").ok();
    });
}

pub fn create_test_app() -> Router {
    init();
    football_group_manager::create_app_lazy(
        std::env::var("DATABASE_URL").unwrap(),
        std::env::var("MAX_DB_CONNECTIONS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10),
    )
}

pub async fn parse_response_body<T: serde::de::DeserializeOwned>(
    response: Response<Body>
) -> T {
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let body = String::from_utf8(bytes.to_vec()).unwrap();
    serde_json::from_str(&body).expect("Failed to parse response body")
}