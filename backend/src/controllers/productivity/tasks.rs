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
    models::{productivity::Task, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateTaskDto {
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub status: String,
    pub reminder_time: Option<DateTime<Utc>>,
    pub goal_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateTaskDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<i32>,
    pub due_date: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub reminder_time: Option<DateTime<Utc>>,
    pub goal_id: Option<Uuid>,
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateTaskDto>,
) -> Result<(StatusCode, Json<Task>), (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        INSERT INTO tasks (user_id, title, description, priority, due_date, status, reminder_time, goal_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, user_id, title, description, priority, due_date, status, reminder_time, goal_id, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.priority)
    .bind(payload.due_date)
    .bind(&payload.status)
    .bind(payload.reminder_time)
    .bind(payload.goal_id)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(task)))
}

pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<Task>>, (StatusCode, String)> {
    let tasks = sqlx::query_as::<_, Task>(
        "SELECT id, user_id, title, description, priority, due_date, status, reminder_time, goal_id, created_at, updated_at FROM tasks WHERE user_id = $1 ORDER BY due_date ASC NULLS LAST",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(tasks))
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT id, user_id, title, description, priority, due_date, status, reminder_time, goal_id, created_at, updated_at FROM tasks WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match task {
        Some(t) => Ok(Json(t)),
        None => Err((StatusCode::NOT_FOUND, "Task not found".to_string())),
    }
}

pub async fn update_task(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTaskDto>,
) -> Result<Json<Task>, (StatusCode, String)> {
    let task = sqlx::query_as::<_, Task>(
        r#"
        UPDATE tasks
        SET title = COALESCE($1, title),
            description = COALESCE($2, description),
            priority = COALESCE($3, priority),
            due_date = COALESCE($4, due_date),
            status = COALESCE($5, status),
            reminder_time = COALESCE($6, reminder_time),
            goal_id = COALESCE($7, goal_id)
        WHERE id = $8 AND user_id = $9
        RETURNING id, user_id, title, description, priority, due_date, status, reminder_time, goal_id, created_at, updated_at
        "#,
    )
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(payload.priority)
    .bind(payload.due_date)
    .bind(&payload.status)
    .bind(payload.reminder_time)
    .bind(payload.goal_id)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match task {
        Some(t) => Ok(Json(t)),
        None => Err((StatusCode::NOT_FOUND, "Task not found".to_string())),
    }
}

pub async fn delete_task(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Task not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
