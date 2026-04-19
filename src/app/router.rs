
use axum::{routing::get, Router, Json};
use serde::Serialize;
use crate::app::state::AppState;
use crate::routes::users;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .nest("/users", users::routes())
        .with_state(state)
}

async fn hello_world() -> Json<HelloResponse> {
    Json(
        HelloResponse {
            message: "Hello World!".into(),
        }
    )
}