use football_group_manager::dto::user::CreateUserRequest;

pub fn get_default_users() -> Vec<CreateUserRequest> {
    vec![
        CreateUserRequest {
            name: "John".into(),
            last_name: Some("Doe".into()),
            password: "password123".into(),
        },
        CreateUserRequest {
            name: "Jane".into(),
            last_name: Some("Smith".into()),
            password: "password456".into(),
        },
        CreateUserRequest {
            name: "Marcos".into(),
            last_name: None,
            password: "password789".into(),
        },
    ]
}