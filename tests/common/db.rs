use sqlx::PgPool;

pub async fn user_exists_by_id(pool: &PgPool, user_id: i32) -> bool {
    sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)")
        .bind(user_id)
        .fetch_one(pool)
        .await
        .expect("Failed to query users table")
}


pub async fn get_users_count(pool: &PgPool) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await
        .expect("Failed to query users table")
}

pub async fn create_user(
    pool: &PgPool,
    username: &str,
    password: &str,
    name: Option<&str>,
    last_name: Option<&str>,
) -> i32 {
    sqlx::query_scalar::<_, i32>(
        "INSERT INTO users (username, name, last_name, password_hash) VALUES ($1, $2, $3, $4) RETURNING id"
    )
        .bind(username)
        .bind(name)
        .bind(last_name)
        .bind(password)
        .fetch_one(pool)
        .await
        .expect("Failed to insert user into database")
}