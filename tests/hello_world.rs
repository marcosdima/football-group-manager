mod common;

use axum::{
    body::Body,
    http::{Request, StatusCode}
};
use tower::ServiceExt;
use serde::Deserialize;

#[derive(Deserialize)]
struct HelloResponse {
    message: String,
}

#[tokio::test]
async fn test_hello() {
    let (app, _) = common::create_test_app().await;

    let response = app
        .oneshot(Request::builder().uri("/hello").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let hello_response: HelloResponse = common::parse_response_body(response).await;
    assert_eq!(hello_response.message, "Hello World!");
}