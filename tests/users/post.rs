use crate::*;
use axum::http::{Method, header::CONTENT_TYPE};

#[tokio::test]
async fn test_create_user() {
    let (app, pool) = common::create_test_app().await;
    let default_data = &default_data::get_default_users()[0];

    let request_body = Body::from(serde_json::to_string(default_data).unwrap());

    let response = app
        .clone()
        .oneshot(
            Request::builder()
            .method(Method::POST)
            .uri("/users/register")
            .header(CONTENT_TYPE, "application/json")
            .body(request_body)
            .unwrap()
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let parsed_response: UserResponse = common::parse_response_body(response).await;
    
    assert_eq!(parsed_response.name, default_data.name);
    assert_eq!(parsed_response.last_name, default_data.last_name);
    assert!(parsed_response.id > 0);

    assert!(db::user_exists_by_id(&pool, parsed_response.id).await);
    assert_eq!(db::get_users_count(&pool).await, 1);
}