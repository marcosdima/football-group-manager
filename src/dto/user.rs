use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserResponse {
    pub id: i32,
    pub username: String,
    pub name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub username: String,
    pub name: Option<String>,
    pub last_name: Option<String>,
    pub password: String,
}