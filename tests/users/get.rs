use crate::*;

#[derive(Deserialize)]
struct User {
    id: i32,
    name: String,
    last_name: Option<String>,
}

#[tokio::test]
async fn test_get_users() {
    let app = common::create_test_app();
    let default_data = default_data::get_default_users();

    let response = app
        .oneshot(Request::builder().uri("/users").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let users: Vec<User> = common::parse_response_body(response).await;
    assert_eq!(users.len(), 3);
    for i in 0..users.len() {
        let user = &users[i];
        assert_eq!(default_data[i].name, user.name);
        assert_eq!(default_data[i].last_name, user.last_name);
    }
    assert_eq!(users[0].id, 1);
    assert_eq!(users[0].name, "John");
    assert_eq!(users[0].last_name, Some("Doe".into()));
    assert_eq!(users[1].id, 2);
    assert_eq!(users[1].name, "Jane");
    assert_eq!(users[1].last_name, Some("Smith".into()));
    assert_eq!(users[2].id, 3);
    assert_eq!(users[2].name, "Marcos");
    assert_eq!(users[2].last_name, None);
}