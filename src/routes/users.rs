use axum::{
    Router,
    routing::get,
    Json,
};

use crate::app::state::AppState;
use crate::dto::user::GetUserResponse;


pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_users))
}

async fn list_users() -> Json<Vec<GetUserResponse>> {
    Json(vec![
        GetUserResponse {
            id: 1,
            name: "John".into(),
            last_name: Some("Doe".into()),
        },
        GetUserResponse {
            id: 2,
            name: "Jane".into(),
            last_name: Some("Smith".into()),
        },
        GetUserResponse {
            id: 3,
            name: "Marcos".into(),
            last_name: None,
        }
    ])
}