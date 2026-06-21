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
    models::{lifestyle::ExerciseLog, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateExerciseLogDto {
    pub date: DateTime<Utc>,
    pub r#type: String,
    pub duration_minutes: i32,
    pub calories_burned: Option<f64>,
}

#[derive(Deserialize)]
pub struct UpdateExerciseLogDto {
    pub date: Option<DateTime<Utc>>,
    pub r#type: Option<String>,
    pub duration_minutes: Option<i32>,
    pub calories_burned: Option<f64>,
}

pub async fn create_exercise_log(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateExerciseLogDto>,
) -> Result<(StatusCode, Json<ExerciseLog>), (StatusCode, String)> {
    let log = sqlx::query_as::<_, ExerciseLog>(
        r#"
        INSERT INTO exercise_logs (user_id, date, type, duration_minutes, calories_burned)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, user_id, date, type, duration_minutes, calories_burned, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(payload.date)
    .bind(&payload.r#type)
    .bind(payload.duration_minutes)
    .bind(payload.calories_burned)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(log)))
}

pub async fn list_exercise_logs(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<ExerciseLog>>, (StatusCode, String)> {
    let logs = sqlx::query_as::<_, ExerciseLog>(
        "SELECT id, user_id, date, type, duration_minutes, calories_burned, created_at, updated_at FROM exercise_logs WHERE user_id = $1 ORDER BY date DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(logs))
}

pub async fn get_exercise_log(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<ExerciseLog>, (StatusCode, String)> {
    let log = sqlx::query_as::<_, ExerciseLog>(
        "SELECT id, user_id, date, type, duration_minutes, calories_burned, created_at, updated_at FROM exercise_logs WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match log {
        Some(l) => Ok(Json(l)),
        None => Err((StatusCode::NOT_FOUND, "Exercise log not found".to_string())),
    }
}

pub async fn update_exercise_log(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateExerciseLogDto>,
) -> Result<Json<ExerciseLog>, (StatusCode, String)> {
    let log = sqlx::query_as::<_, ExerciseLog>(
        r#"
        UPDATE exercise_logs
        SET date = COALESCE($1, date),
            type = COALESCE($2, type),
            duration_minutes = COALESCE($3, duration_minutes),
            calories_burned = COALESCE($4, calories_burned)
        WHERE id = $5 AND user_id = $6
        RETURNING id, user_id, date, type, duration_minutes, calories_burned, created_at, updated_at
        "#,
    )
    .bind(payload.date)
    .bind(&payload.r#type)
    .bind(payload.duration_minutes)
    .bind(payload.calories_burned)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match log {
        Some(l) => Ok(Json(l)),
        None => Err((StatusCode::NOT_FOUND, "Exercise log not found".to_string())),
    }
}

pub async fn delete_exercise_log(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM exercise_logs WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Exercise log not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
