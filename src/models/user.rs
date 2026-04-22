use sqlx::FromRow;

#[derive(FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub last_name: Option<String>,
    pub password_hash: String,
}