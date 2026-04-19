use serde::Serialize;

#[derive(Serialize)]
pub struct GetUserResponse {
    pub id: i32,
    pub name: String,
    pub last_name: Option<String>,
}

#[derive(Serialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub last_name: Option<String>,
    pub password: String,
}