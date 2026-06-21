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
    models::{productivity::Goal, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateGoalDto {
    pub name: String,
    pub description: Option<String>,
    pub target_date: Option<DateTime<Utc>>,
    pub progress: Option<f64>,
    pub r#type: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateGoalDto {
    pub name: Option<String>,
    pub description: Option<String>,
    pub target_date: Option<DateTime<Utc>>,
    pub progress: Option<f64>,
    pub r#type: Option<String>,
}

pub async fn create_goal(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateGoalDto>,
) -> Result<(StatusCode, Json<Goal>), (StatusCode, String)> {
    let progress = payload.progress.unwrap_or(0.0);
    let goal = sqlx::query_as::<_, Goal>(
        r#"
        INSERT INTO goals (user_id, name, description, target_date, progress, type)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, user_id, name, description, target_date, progress, type, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.target_date)
    .bind(progress)
    .bind(&payload.r#type)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(goal)))
}

pub async fn list_goals(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<Goal>>, (StatusCode, String)> {
    let goals = sqlx::query_as::<_, Goal>(
        "SELECT id, user_id, name, description, target_date, progress, type, created_at, updated_at FROM goals WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(goals))
}

pub async fn get_goal(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<Goal>, (StatusCode, String)> {
    let goal = sqlx::query_as::<_, Goal>(
        "SELECT id, user_id, name, description, target_date, progress, type, created_at, updated_at FROM goals WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match goal {
        Some(g) => Ok(Json(g)),
        None => Err((StatusCode::NOT_FOUND, "Goal not found".to_string())),
    }
}

pub async fn update_goal(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateGoalDto>,
) -> Result<Json<Goal>, (StatusCode, String)> {
    let goal = sqlx::query_as::<_, Goal>(
        r#"
        UPDATE goals
        SET name = COALESCE($1, name),
            description = COALESCE($2, description),
            target_date = COALESCE($3, target_date),
            progress = COALESCE($4, progress),
            type = COALESCE($5, type)
        WHERE id = $6 AND user_id = $7
        RETURNING id, user_id, name, description, target_date, progress, type, created_at, updated_at
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.target_date)
    .bind(payload.progress)
    .bind(&payload.r#type)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match goal {
        Some(g) => Ok(Json(g)),
        None => Err((StatusCode::NOT_FOUND, "Goal not found".to_string())),
    }
}

pub async fn delete_goal(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM goals WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Goal not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
