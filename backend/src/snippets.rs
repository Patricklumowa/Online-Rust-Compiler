use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;
use crate::{auth::Claims, db::AppState};

#[derive(Serialize, FromRow, ToSchema)]
pub struct Snippet {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub code: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateSnippetRequest {
    pub title: String,
    pub code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct UpdateSnippetRequest {
    pub title: String,
    pub code: String,
}

#[derive(Deserialize, ToSchema)]
pub struct PatchSnippetRequest {
    pub title: Option<String>,
    pub code: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct CreateSnippetResponse {
    pub id: String,
}

// POST /snippets
#[utoipa::path(
    post,
    path = "/snippets",
    request_body = CreateSnippetRequest,
    responses(
        (status = 201, description = "Snippet created successfully", body = CreateSnippetResponse),
        (status = 401, description = "Unauthorized")
    ),
    tag = "snippets",
    security(
        ("jwt" = [])
    )
)]
pub async fn create_snippet(
    State(state): State<AppState>,
    claims: Claims,
    Json(payload): Json<CreateSnippetRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = uuid::Uuid::new_v4().to_string();
    
    sqlx::query("INSERT INTO snippets (id, user_id, title, code) VALUES (?, ?, ?, ?)")
        .bind(&id)
        .bind(&claims.sub)
        .bind(&payload.title)
        .bind(&payload.code)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(CreateSnippetResponse { id })))
}

// GET /snippets
#[utoipa::path(
    get,
    path = "/snippets",
    responses(
        (status = 200, description = "List of user snippets", body = Vec<Snippet>),
        (status = 401, description = "Unauthorized")
    ),
    tag = "snippets",
    security(
        ("jwt" = [])
    )
)]
pub async fn list_snippets(
    State(state): State<AppState>,
    claims: Claims,
) -> Result<Json<Vec<Snippet>>, (StatusCode, String)> {
    let snippets = sqlx::query_as::<_, Snippet>("SELECT * FROM snippets WHERE user_id = ? ORDER BY updated_at DESC")
        .bind(&claims.sub)
        .fetch_all(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(snippets))
}

// GET /snippets/:id
#[utoipa::path(
    get,
    path = "/snippets/{id}",
    params(
        ("id" = String, Path, description = "Snippet ID")
    ),
    responses(
        (status = 200, description = "Snippet details", body = Snippet),
        (status = 404, description = "Snippet not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "snippets",
    security(
        ("jwt" = [])
    )
)]
pub async fn get_snippet(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<Json<Snippet>, (StatusCode, String)> {
    let snippet = sqlx::query_as::<_, Snippet>("SELECT * FROM snippets WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match snippet {
        Some(s) => Ok(Json(s)),
        None => Err((StatusCode::NOT_FOUND, "Snippet not found".to_string())),
    }
}

// PUT /snippets/:id
#[utoipa::path(
    put,
    path = "/snippets/{id}",
    params(
        ("id" = String, Path, description = "Snippet ID")
    ),
    request_body = UpdateSnippetRequest,
    responses(
        (status = 200, description = "Snippet updated successfully"),
        (status = 404, description = "Snippet not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "snippets",
    security(
        ("jwt" = [])
    )
)]
pub async fn update_snippet(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSnippetRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("UPDATE snippets SET title = ?, code = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ? AND user_id = ?")
        .bind(&payload.title)
        .bind(&payload.code)
        .bind(&id)
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Snippet not found or unauthorized".to_string()));
    }

    Ok(StatusCode::OK)
}

// PATCH /snippets/:id
#[utoipa::path(
    patch,
    path = "/snippets/{id}",
    params(
        ("id" = String, Path, description = "Snippet ID")
    ),
    request_body = PatchSnippetRequest,
    responses(
        (status = 200, description = "Snippet patched successfully"),
        (status = 404, description = "Snippet not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "snippets",
    security(
        ("jwt" = [])
    )
)]
pub async fn patch_snippet(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
    Json(payload): Json<PatchSnippetRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // We need to build the query dynamically or just fetch, update, save.
    // Fetching first is safer for partial updates to ensure we don't overwrite with nulls if we were using a struct that had all fields.
    // But here we can use COALESCE in SQL or dynamic query building.
    // Let's use a simple approach: Fetch, modify, update.

    let mut snippet = sqlx::query_as::<_, Snippet>("SELECT * FROM snippets WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&claims.sub)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Snippet not found".to_string()))?;

    if let Some(title) = payload.title {
        snippet.title = title;
    }
    if let Some(code) = payload.code {
        snippet.code = code;
    }

    sqlx::query("UPDATE snippets SET title = ?, code = ?, updated_at = CURRENT_TIMESTAMP WHERE id = ?")
        .bind(&snippet.title)
        .bind(&snippet.code)
        .bind(&id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

// DELETE /snippets/:id
#[utoipa::path(
    delete,
    path = "/snippets/{id}",
    params(
        ("id" = String, Path, description = "Snippet ID")
    ),
    responses(
        (status = 204, description = "Snippet deleted successfully"),
        (status = 404, description = "Snippet not found"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "snippets",
    security(
        ("jwt" = [])
    )
)]
pub async fn delete_snippet(
    State(state): State<AppState>,
    claims: Claims,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM snippets WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Snippet not found or unauthorized".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
