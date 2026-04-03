use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub completed: bool,
    #[serde(rename = "order")]
    pub display_order: i64,
}

#[derive(Debug, Serialize)]
pub struct CreateTodo {
    pub title: String,
}

#[derive(Debug, Serialize, Default)]
pub struct UpdateTodo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct ToggleAll {
    pub completed: bool,
}
