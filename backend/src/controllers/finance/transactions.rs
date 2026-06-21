use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::{finance::Transaction, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateTransactionDto {
    pub account_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub amount: f64,
    pub date: DateTime<Utc>,
    pub r#type: String,
    pub recurring: Option<bool>,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateTransactionDto {
    pub account_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub date: Option<DateTime<Utc>>,
    pub r#type: Option<String>,
    pub recurring: Option<bool>,
    pub description: Option<String>,
}

pub async fn create_transaction(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateTransactionDto>,
) -> Result<(StatusCode, Json<Transaction>), (StatusCode, String)> {
    let recurring = payload.recurring.unwrap_or(false);
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        INSERT INTO transactions (user_id, account_id, category_id, amount, date, type, recurring, description)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, account_id, category_id, amount, date, type, recurring, description, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(payload.account_id)
    .bind(payload.category_id)
    .bind(payload.amount)
    .bind(payload.date)
    .bind(&payload.r#type)
    .bind(recurring)
    .bind(&payload.description)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(transaction)))
}

pub async fn list_transactions(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<Transaction>>, (StatusCode, String)> {
    let transactions = sqlx::query_as::<_, Transaction>(
        "SELECT id, user_id, account_id, category_id, amount, date, type, recurring, description, created_at, updated_at FROM transactions WHERE user_id = $1 ORDER BY date DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(transactions))
}

pub async fn get_transaction(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<Transaction>, (StatusCode, String)> {
    let transaction = sqlx::query_as::<_, Transaction>(
        "SELECT id, user_id, account_id, category_id, amount, date, type, recurring, description, created_at, updated_at FROM transactions WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match transaction {
        Some(t) => Ok(Json(t)),
        None => Err((StatusCode::NOT_FOUND, "Transaction not found".to_string())),
    }
}

pub async fn update_transaction(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionDto>,
) -> Result<Json<Transaction>, (StatusCode, String)> {
    let transaction = sqlx::query_as::<_, Transaction>(
        r#"
        UPDATE transactions
        SET account_id = COALESCE($1, account_id),
            category_id = COALESCE($2, category_id),
            amount = COALESCE($3, amount),
            date = COALESCE($4, date),
            type = COALESCE($5, type),
            recurring = COALESCE($6, recurring),
            description = COALESCE($7, description)
        WHERE id = $8 AND user_id = $9
        RETURNING id, user_id, account_id, category_id, amount, date, type, recurring, description, created_at, updated_at
        "#,
    )
    .bind(payload.account_id)
    .bind(payload.category_id)
    .bind(payload.amount)
    .bind(payload.date)
    .bind(&payload.r#type)
    .bind(payload.recurring)
    .bind(&payload.description)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match transaction {
        Some(t) => Ok(Json(t)),
        None => Err((StatusCode::NOT_FOUND, "Transaction not found".to_string())),
    }
}

pub async fn delete_transaction(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM transactions WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Transaction not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
