use sqlx::{PgPool, FromRow};
use super::error::AppError; // Use your AppError type

// Mirror the users table structure
#[derive(FromRow)]
pub struct User {
    pub id: i64, // Or Uuid if using UUIDs
    pub username: String,
    pub email: String,
    pub password_hash: String,
    // pub created_at: chrono::DateTime<chrono::Utc>,
}

// Example function to create a user
pub async fn create_user(pool: &PgPool, username: &str, email: &str, password_hash: &str) -> Result<i64, AppError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        username,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await
    .map_err(|e| {
        // Handle specific errors like unique constraint violation
        if let sqlx::Error::Database(db_err) = &e {
            if db_err.is_unique_violation() {
                return AppError::Conflict("Username or email already exists".to_string());
            }
        }
        AppError::DatabaseError(e)
    })?;

    Ok(result.id)
}

// Example function to find a user by username
pub async fn find_user_by_username(pool: &PgPool, username: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, email, password_hash FROM users WHERE username = $1
        "#,
        username
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(user)
}

// --- Add more database functions ---
// - find_user_by_id
// - update_player_state
// - get_entities_in_region
// - save_map_chunk_modifications
// - etc.
