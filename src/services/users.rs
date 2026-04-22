use sqlx;
use crate::models::user::User;

pub async fn create_user(
    db: &sqlx::PgPool,
    name: String,
    last_name: Option<String>,
    password: &str,
) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        "INSERT INTO users (name, last_name, password_hash)
         VALUES ($1, $2, $3)
         RETURNING id, name, last_name, password_hash",
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