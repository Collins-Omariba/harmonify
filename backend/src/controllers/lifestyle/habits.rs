use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::{lifestyle::Habit, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateHabitDto {
    pub name: String,
    pub frequency: String,
}

#[derive(Deserialize)]
pub struct UpdateHabitDto {
    pub name: Option<String>,
    pub frequency: Option<String>,
}

pub async fn create_habit(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateHabitDto>,
) -> Result<(StatusCode, Json<Habit>), (StatusCode, String)> {
    let habit = sqlx::query_as::<_, Habit>(
        r#"
        INSERT INTO habits (user_id, name, frequency)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, name, frequency, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(&payload.name)
    .bind(&payload.frequency)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(habit)))
}

pub async fn list_habits(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<Habit>>, (StatusCode, String)> {
    let habits = sqlx::query_as::<_, Habit>(
        "SELECT id, user_id, name, frequency, created_at, updated_at FROM habits WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(habits))
}

pub async fn get_habit(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<Habit>, (StatusCode, String)> {
    let habit = sqlx::query_as::<_, Habit>(
        "SELECT id, user_id, name, frequency, created_at, updated_at FROM habits WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match habit {
        Some(h) => Ok(Json(h)),
        None => Err((StatusCode::NOT_FOUND, "Habit not found".to_string())),
    }
}

pub async fn update_habit(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateHabitDto>,
) -> Result<Json<Habit>, (StatusCode, String)> {
    let habit = sqlx::query_as::<_, Habit>(
        r#"
        UPDATE habits
        SET name = COALESCE($1, name),
            frequency = COALESCE($2, frequency)
        WHERE id = $3 AND user_id = $4
        RETURNING id, user_id, name, frequency, created_at, updated_at
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.frequency)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match habit {
        Some(h) => Ok(Json(h)),
        None => Err((StatusCode::NOT_FOUND, "Habit not found".to_string())),
    }
}

pub async fn delete_habit(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM habits WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Habit not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
