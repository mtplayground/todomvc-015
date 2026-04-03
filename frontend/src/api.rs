use gloo_net::http::Request;

use crate::models::{CreateTodo, Todo, ToggleAll, UpdateTodo};

const BASE_URL: &str = "/api/todos";

#[derive(Debug, Clone)]
pub struct ApiError(pub String);

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub async fn fetch_todos() -> Result<Vec<Todo>, ApiError> {
    let resp = Request::get(BASE_URL)
        .send()
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    if !resp.ok() {
        return Err(ApiError(format!("HTTP {}", resp.status())));
    }

    resp.json::<Vec<Todo>>()
        .await
        .map_err(|e| ApiError(e.to_string()))
}

pub async fn create_todo(input: &CreateTodo) -> Result<Todo, ApiError> {
    let resp = Request::post(BASE_URL)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(input).map_err(|e| ApiError(e.to_string()))?)
        .map_err(|e| ApiError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    if !resp.ok() {
        return Err(ApiError(format!("HTTP {}", resp.status())));
    }

    resp.json::<Todo>()
        .await
        .map_err(|e| ApiError(e.to_string()))
}

pub async fn update_todo(id: &str, input: &UpdateTodo) -> Result<Todo, ApiError> {
    let url = format!("{}/{}", BASE_URL, id);
    let resp = Request::patch(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(input).map_err(|e| ApiError(e.to_string()))?)
        .map_err(|e| ApiError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    if !resp.ok() {
        return Err(ApiError(format!("HTTP {}", resp.status())));
    }

    resp.json::<Todo>()
        .await
        .map_err(|e| ApiError(e.to_string()))
}

pub async fn delete_todo(id: &str) -> Result<(), ApiError> {
    let url = format!("{}/{}", BASE_URL, id);
    let resp = Request::delete(&url)
        .send()
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    if !resp.ok() {
        return Err(ApiError(format!("HTTP {}", resp.status())));
    }

    Ok(())
}

pub async fn delete_completed() -> Result<(), ApiError> {
    let url = format!("{}/completed", BASE_URL);
    let resp = Request::delete(&url)
        .send()
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    if !resp.ok() {
        return Err(ApiError(format!("HTTP {}", resp.status())));
    }

    Ok(())
}

pub async fn toggle_all(completed: bool) -> Result<Vec<Todo>, ApiError> {
    let url = format!("{}/toggle-all", BASE_URL);
    let input = ToggleAll { completed };
    let resp = Request::patch(&url)
        .header("Content-Type", "application/json")
        .body(serde_json::to_string(&input).map_err(|e| ApiError(e.to_string()))?)
        .map_err(|e| ApiError(e.to_string()))?
        .send()
        .await
        .map_err(|e| ApiError(e.to_string()))?;

    if !resp.ok() {
        return Err(ApiError(format!("HTTP {}", resp.status())));
    }

    resp.json::<Vec<Todo>>()
        .await
        .map_err(|e| ApiError(e.to_string()))
}
