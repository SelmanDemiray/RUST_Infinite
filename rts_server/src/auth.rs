use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use super::AppState;
use super::db; // To interact with the database
use super::error::AppError;
// use argon2::{self, Config}; // For password hashing
// use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password_hash: String, // Ideally, hash this server-side from a plain password
}

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password_hash: String, // Ideally, hash this server-side from a plain password
}

#[derive(Serialize)]
pub struct AuthResponse {
    user_id: i64, // Or Uuid if using UUIDs
    token: String, // JWT or session token
}

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<StatusCode, AppError> {
    tracing::info!("Register attempt for user: {}", payload.username);

    // --- Basic Validation ---
    if payload.username.is_empty() || payload.password_hash.is_empty() || payload.email.is_empty() {
         return Err(AppError::BadRequest("Missing fields".to_string()));
    }

    // --- Password Hashing (Example - Do this properly!) ---
    // let salt = Uuid::new_v4().to_string(); // Generate unique salt per user
    // let config = Config::default();
    // let hashed_password = argon2::hash_encoded(payload.password.as_bytes(), salt.as_bytes(), &config)
    //     .map_err(|e| AppError::InternalServerError(format!("Password hashing failed: {}", e)))?;

    // --- Database Interaction ---
    let _user_id = db::create_user(&state.db_pool, &payload.username, &payload.email, &payload.password_hash /* use hashed_password */)
        .await?; // Handle potential db errors (e.g., username exists)

    Ok(StatusCode::CREATED)
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
     tracing::info!("Login attempt for user: {}", payload.username);

    // --- Basic Validation ---
     if payload.username.is_empty() || payload.password_hash.is_empty() {
         return Err(AppError::BadRequest("Missing fields".to_string()));
     }

    // --- Database Interaction ---
    let user = db::find_user_by_username(&state.db_pool, &payload.username).await?;

    // --- Password Verification (Example - Do this properly!) ---
    // let password_matches = argon2::verify_encoded(&user.password_hash, payload.password.as_bytes())
    //     .map_err(|e| AppError::InternalServerError(format!("Password verification failed: {}", e)))?;
    // if !password_matches {
    //     return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    // }
    // --- Simplified check for skeleton ---
    if user.password_hash != payload.password_hash {
         return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }


    // --- Token Generation (Example - Use JWT) ---
    let token = format!("dummy-token-for-{}", user.id); // Replace with actual JWT generation

    Ok(Json(AuthResponse {
        user_id: user.id,
        token,
    }))
}
