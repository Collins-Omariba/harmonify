use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use crate::models::user::User;
use crate::AppState;

#[derive(Deserialize)]
pub struct RegisterDto {
    pub email: String,
    pub password: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginDto {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub user: UserResponse,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: uuid::Uuid,
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterDto>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1")
        .bind(&payload.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if existing_user.is_some() {
        return Err((StatusCode::CONFLICT, "User already exists".to_string()));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(payload.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error hashing password".to_string()))?
        .to_string();

    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (email, password, name)
        VALUES ($1, $2, $3)
        RETURNING id, email, name, password, created_at, updated_at
        "#
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .bind(&payload.name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes())
    ).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error creating token".to_string()))?;

    Ok(Json(AuthResponse {
        access_token: token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }))
}

pub async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, email, name, password, created_at, updated_at FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match user {
        Some(u) => u,
        None => return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string())),
    };

    let parsed_hash = PasswordHash::new(&user.password)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error parsing hash".to_string()))?;
    if Argon2::default().verify_password(payload.password.as_bytes(), &parsed_hash).is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()));
    }

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: (Utc::now() + Duration::days(1)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes())
    ).map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Error creating token".to_string()))?;

    Ok(Json(AuthResponse {
        access_token: token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        }
    }))
}

pub async fn get_profile(current_user: User) -> Json<User> {
    Json(current_user)
}
