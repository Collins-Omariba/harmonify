use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;
use crate::models::user::User;
use crate::controllers::auth::Claims;
use crate::AppState;

impl FromRequestParts<Arc<AppState>> for User {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(parts: &mut Parts, state: &Arc<AppState>) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .filter(|h| h.starts_with("Bearer "));

        let token = match auth_header {
            Some(header) => header.trim_start_matches("Bearer "),
            None => return Err((StatusCode::UNAUTHORIZED, "Missing token".to_string())),
        };

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default()
        ).map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token".to_string()))?;

        let user_id = uuid::Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token sub".to_string()))?;

        let user = sqlx::query_as::<_, User>(
            "SELECT id, email, name, password, created_at, updated_at FROM users WHERE id = $1"
        )
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        match user {
            Some(u) => Ok(u),
            None => Err((StatusCode::UNAUTHORIZED, "User not found".to_string())),
        }
    }
}
