use axum::{
    Router,
    routing::{get, post},
    Json,
    http::StatusCode,
};
use axum::extract::State;
use crate::app::state::AppState;
use crate::dto::user::{UserResponse, CreateUserRequest};
use crate::utils::functions::hash_password;
use crate::services;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
        .route("/register", post(create_user))
}

async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let password = hash_password(&req.password);

    let created = services::users::create_user(
        &state.db,
        &req.username,
        &password,
        req.name.as_deref(),
        req.last_name.as_deref(),
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(UserResponse {
            id: created.id,
            username: created.username,
            name: created.name,
            last_name: created.last_name,
        }),
    ))
}

async fn list_users(
    State(state): State<AppState>,
) -> Json<Vec<UserResponse>> {
    let users = services::users::get_users(&state.db)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            username: u.username,
            name: u.name,
            last_name: u.last_name,
        })
        .collect();
    Json(users)
}