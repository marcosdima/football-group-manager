use football_group_manager::dto::user::CreateUserRequest;

pub fn get_default_users() -> Vec<CreateUserRequest> {
    vec![
        CreateUserRequest {
            username: "john_doe".into(),
            password: "password123".into(),
            name: Some("John".into()),
            last_name: Some("Doe".into()),
        },
        CreateUserRequest {
            username: "jane_smith".into(),
            password: "password456".into(),
            name: Some("Jane".into()),
            last_name: Some("Smith".into()),
        },
        CreateUserRequest {
            username: "marcos".into(),
            password: "password789".into(),
            name: Some("Marcos".into()),
            last_name: None,
        },
    ]
}