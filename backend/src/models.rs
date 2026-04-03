use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    #[serde(rename = "order")]
    pub display_order: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    #[serde(default)]
    pub order: Option<i64>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub order: Option<i64>,
}
