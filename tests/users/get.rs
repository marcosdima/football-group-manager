use football_group_manager::dto::user::UserResponse;

use crate::*;

#[tokio::test]
async fn test_get_users() {
    let (app, pool) = common::create_test_app().await;
    let default_data = default_data::get_default_users();

    for user_data in &default_data {
        db::create_user(
            &pool,
            &user_data.name,
            user_data.last_name.as_deref(),
            &user_data.password
        ).await;
    }

    let response = app
        .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let users: Vec<UserResponse> = common::parse_response_body(response).await;
    assert_eq!(users.len(), 3);

    for i in 0..users.len() {
        let user = &users[i];
        assert_eq!(default_data[i].name, user.name);
        assert_eq!(default_data[i].last_name, user.last_name);
    }
}