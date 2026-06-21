use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::{finance::Account, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateAccountDto {
    pub name: String,
    pub r#type: String,
    pub balance: Option<f64>,
}

#[derive(Deserialize)]
pub struct UpdateAccountDto {
    pub name: Option<String>,
    pub r#type: Option<String>,
    pub balance: Option<f64>,
}

pub async fn create_account(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateAccountDto>,
) -> Result<(StatusCode, Json<Account>), (StatusCode, String)> {
    let balance = payload.balance.unwrap_or(0.0);
    let account = sqlx::query_as::<_, Account>(
        r#"
        INSERT INTO accounts (user_id, name, type, balance)
        VALUES ($1, $2, $3, $4)
        RETURNING id, user_id, name, type, balance, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(&payload.name)
    .bind(&payload.r#type)
    .bind(balance)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(account)))
}

pub async fn list_accounts(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<Account>>, (StatusCode, String)> {
    let accounts = sqlx::query_as::<_, Account>(
        "SELECT id, user_id, name, type, balance, created_at, updated_at FROM accounts WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(accounts))
}

pub async fn get_account(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<Account>, (StatusCode, String)> {
    let account = sqlx::query_as::<_, Account>(
        "SELECT id, user_id, name, type, balance, created_at, updated_at FROM accounts WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match account {
        Some(acc) => Ok(Json(acc)),
        None => Err((StatusCode::NOT_FOUND, "Account not found".to_string())),
    }
}

pub async fn update_account(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateAccountDto>,
) -> Result<Json<Account>, (StatusCode, String)> {
    let account = sqlx::query_as::<_, Account>(
        r#"
        UPDATE accounts
        SET name = COALESCE($1, name),
            type = COALESCE($2, type),
            balance = COALESCE($3, balance)
        WHERE id = $4 AND user_id = $5
        RETURNING id, user_id, name, type, balance, created_at, updated_at
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.r#type)
    .bind(payload.balance)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match account {
        Some(acc) => Ok(Json(acc)),
        None => Err((StatusCode::NOT_FOUND, "Account not found".to_string())),
    }
}

pub async fn delete_account(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM accounts WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Account not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
