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
    models::{lifestyle::JournalEntry, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateJournalEntryDto {
    pub content: String,
    pub date: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct UpdateJournalEntryDto {
    pub content: Option<String>,
    pub date: Option<DateTime<Utc>>,
}

pub async fn create_journal_entry(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateJournalEntryDto>,
) -> Result<(StatusCode, Json<JournalEntry>), (StatusCode, String)> {
    let entry = sqlx::query_as::<_, JournalEntry>(
        r#"
        INSERT INTO journal_entries (user_id, content, date)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, content, date, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(&payload.content)
    .bind(payload.date)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(entry)))
}

pub async fn list_journal_entries(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<JournalEntry>>, (StatusCode, String)> {
    let entries = sqlx::query_as::<_, JournalEntry>(
        "SELECT id, user_id, content, date, created_at, updated_at FROM journal_entries WHERE user_id = $1 ORDER BY date DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(entries))
}

pub async fn get_journal_entry(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<JournalEntry>, (StatusCode, String)> {
    let entry = sqlx::query_as::<_, JournalEntry>(
        "SELECT id, user_id, content, date, created_at, updated_at FROM journal_entries WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match entry {
        Some(e) => Ok(Json(e)),
        None => Err((StatusCode::NOT_FOUND, "Journal entry not found".to_string())),
    }
}

pub async fn update_journal_entry(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateJournalEntryDto>,
) -> Result<Json<JournalEntry>, (StatusCode, String)> {
    let entry = sqlx::query_as::<_, JournalEntry>(
        r#"
        UPDATE journal_entries
        SET content = COALESCE($1, content),
            date = COALESCE($2, date)
        WHERE id = $3 AND user_id = $4
        RETURNING id, user_id, content, date, created_at, updated_at
        "#,
    )
    .bind(&payload.content)
    .bind(payload.date)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match entry {
        Some(e) => Ok(Json(e)),
        None => Err((StatusCode::NOT_FOUND, "Journal entry not found".to_string())),
    }
}

pub async fn delete_journal_entry(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM journal_entries WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Journal entry not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
