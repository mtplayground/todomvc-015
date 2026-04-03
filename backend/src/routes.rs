use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch},
    Json, Router,
};
use serde::Deserialize;

use crate::db::DbPool;
use crate::models::{CreateTodo, UpdateTodo};
use crate::repo;

pub fn api_router(pool: DbPool) -> Router {
    Router::new()
        .route("/api/todos", get(list_todos).post(create_todo))
        .route("/api/todos/completed", delete(delete_completed))
        .route("/api/todos/toggle-all", patch(toggle_all))
        .route(
            "/api/todos/:id",
            get(get_todo).patch(update_todo).delete(delete_todo),
        )
        .with_state(pool)
}

async fn list_todos(State(pool): State<DbPool>) -> impl IntoResponse {
    match repo::list_all(&pool) {
        Ok(todos) => Json(todos).into_response(),
        Err(e) => {
            tracing::error!("list_all failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn create_todo(
    State(pool): State<DbPool>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    if input.title.trim().is_empty() {
        return (StatusCode::BAD_REQUEST, "title must not be empty").into_response();
    }
    match repo::create(&pool, &input) {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => {
            tracing::error!("create failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn get_todo(State(pool): State<DbPool>, Path(id): Path<String>) -> impl IntoResponse {
    match repo::get_by_id(&pool, &id) {
        Ok(Some(todo)) => Json(todo).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("get_by_id failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn update_todo(
    State(pool): State<DbPool>,
    Path(id): Path<String>,
    Json(input): Json<UpdateTodo>,
) -> impl IntoResponse {
    match repo::update(&pool, &id, &input) {
        Ok(Some(todo)) => Json(todo).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("update failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn delete_todo(State(pool): State<DbPool>, Path(id): Path<String>) -> impl IntoResponse {
    match repo::delete(&pool, &id) {
        Ok(true) => StatusCode::NO_CONTENT.into_response(),
        Ok(false) => StatusCode::NOT_FOUND.into_response(),
        Err(e) => {
            tracing::error!("delete failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

async fn delete_completed(State(pool): State<DbPool>) -> impl IntoResponse {
    match repo::delete_completed(&pool) {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(e) => {
            tracing::error!("delete_completed failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Deserialize)]
struct ToggleAllInput {
    completed: bool,
}

async fn toggle_all(
    State(pool): State<DbPool>,
    Json(input): Json<ToggleAllInput>,
) -> impl IntoResponse {
    match repo::toggle_all(&pool, input.completed) {
        Ok(todos) => Json(todos).into_response(),
        Err(e) => {
            tracing::error!("toggle_all failed: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}
