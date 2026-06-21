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
    models::{finance::Budget, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateBudgetDto {
    pub category_id: Uuid,
    pub amount: f64,
    pub period: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct UpdateBudgetDto {
    pub category_id: Option<Uuid>,
    pub amount: Option<f64>,
    pub period: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

pub async fn create_budget(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateBudgetDto>,
) -> Result<(StatusCode, Json<Budget>), (StatusCode, String)> {
    let budget = sqlx::query_as::<_, Budget>(
        r#"
        INSERT INTO budgets (user_id, category_id, amount, period, start_date, end_date)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, category_id, amount, period, start_date, end_date, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(payload.category_id)
    .bind(payload.amount)
    .bind(&payload.period)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(budget)))
}

pub async fn list_budgets(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<Budget>>, (StatusCode, String)> {
    let budgets = sqlx::query_as::<_, Budget>(
        "SELECT id, user_id, category_id, amount, period, start_date, end_date, created_at, updated_at FROM budgets WHERE user_id = $1 ORDER BY start_date DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(budgets))
}

pub async fn get_budget(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<Budget>, (StatusCode, String)> {
    let budget = sqlx::query_as::<_, Budget>(
        "SELECT id, user_id, category_id, amount, period, start_date, end_date, created_at, updated_at FROM budgets WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match budget {
        Some(b) => Ok(Json(b)),
        None => Err((StatusCode::NOT_FOUND, "Budget not found".to_string())),
    }
}

pub async fn update_budget(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateBudgetDto>,
) -> Result<Json<Budget>, (StatusCode, String)> {
    let budget = sqlx::query_as::<_, Budget>(
        r#"
        UPDATE budgets
        SET category_id = COALESCE($1, category_id),
            amount = COALESCE($2, amount),
            period = COALESCE($3, period),
            start_date = COALESCE($4, start_date),
            end_date = COALESCE($5, end_date)
        WHERE id = $6 AND user_id = $7
        RETURNING id, user_id, category_id, amount, period, start_date, end_date, created_at, updated_at
        "#,
    )
    .bind(payload.category_id)
    .bind(payload.amount)
    .bind(&payload.period)
    .bind(payload.start_date)
    .bind(payload.end_date)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match budget {
        Some(b) => Ok(Json(b)),
        None => Err((StatusCode::NOT_FOUND, "Budget not found".to_string())),
    }
}

pub async fn delete_budget(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM budgets WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Budget not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
