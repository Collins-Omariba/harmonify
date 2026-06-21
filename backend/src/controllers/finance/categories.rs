use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    models::{finance::Category, user::User},
    AppState,
};

#[derive(Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    pub r#type: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateCategoryDto {
    pub name: Option<String>,
    pub r#type: Option<String>,
}

pub async fn create_category(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Json(payload): Json<CreateCategoryDto>,
) -> Result<(StatusCode, Json<Category>), (StatusCode, String)> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (user_id, name, type)
        VALUES ($1, $2, $3)
        RETURNING id, user_id, name, type, created_at, updated_at
        "#,
    )
    .bind(current_user.id)
    .bind(&payload.name)
    .bind(&payload.r#type)
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn list_categories(
    State(state): State<Arc<AppState>>,
    current_user: User,
) -> Result<Json<Vec<Category>>, (StatusCode, String)> {
    let categories = sqlx::query_as::<_, Category>(
        "SELECT id, user_id, name, type, created_at, updated_at FROM categories WHERE user_id = $1 ORDER BY created_at DESC",
    )
    .bind(current_user.id)
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(categories))
}

pub async fn get_category(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<Json<Category>, (StatusCode, String)> {
    let category = sqlx::query_as::<_, Category>(
        "SELECT id, user_id, name, type, created_at, updated_at FROM categories WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match category {
        Some(cat) => Ok(Json(cat)),
        None => Err((StatusCode::NOT_FOUND, "Category not found".to_string())),
    }
}

pub async fn update_category(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCategoryDto>,
) -> Result<Json<Category>, (StatusCode, String)> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories
        SET name = COALESCE($1, name),
            type = COALESCE($2, type)
        WHERE id = $3 AND user_id = $4
        RETURNING id, user_id, name, type, created_at, updated_at
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.r#type)
    .bind(id)
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match category {
        Some(cat) => Ok(Json(cat)),
        None => Err((StatusCode::NOT_FOUND, "Category not found".to_string())),
    }
}

pub async fn delete_category(
    State(state): State<Arc<AppState>>,
    current_user: User,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(current_user.id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        Err((StatusCode::NOT_FOUND, "Category not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
