use sqlx;
use crate::models::user::User;

pub async fn create_user(
    db: &sqlx::PgPool,
    username: &str,
    password: &str,
    name: Option<&str>,
    last_name: Option<&str>,
) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (username, name, last_name, password_hash)
         VALUES ($1, $2, $3, $4)
         RETURNING id, username, name, last_name, password_hash",
        username,
        name,
        last_name,
        password,
    )
    .fetch_one(db)
    .await
}

pub async fn get_users(db: &sqlx::PgPool) -> sqlx::Result<Vec<User>> {
    sqlx::query_as!(
        User,
        "SELECT * FROM users"
    )
    .fetch_all(db)
    .await
}