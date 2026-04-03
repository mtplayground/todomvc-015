use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

use frontend::models::{CreateTodo, Todo, ToggleAll, UpdateTodo};
use frontend::router::Filter;

// --- Filter tests ---

#[wasm_bindgen_test]
fn test_filter_from_hash_all() {
    assert_eq!(Filter::from_hash("#/"), Filter::All);
}

#[wasm_bindgen_test]
fn test_filter_from_hash_active() {
    assert_eq!(Filter::from_hash("#/active"), Filter::Active);
}

#[wasm_bindgen_test]
fn test_filter_from_hash_completed() {
    assert_eq!(Filter::from_hash("#/completed"), Filter::Completed);
}

#[wasm_bindgen_test]
fn test_filter_from_hash_unknown_defaults_to_all() {
    assert_eq!(Filter::from_hash(""), Filter::All);
    assert_eq!(Filter::from_hash("#/unknown"), Filter::All);
    assert_eq!(Filter::from_hash("garbage"), Filter::All);
}

// --- Model serialization tests ---

#[wasm_bindgen_test]
fn test_todo_serialization() {
    let todo = Todo {
        id: "abc-123".into(),
        title: "Test todo".into(),
        completed: false,
        display_order: 1,
    };
    let json = serde_json::to_string(&todo).expect("serialize");
    assert!(json.contains("\"id\":\"abc-123\""));
    assert!(json.contains("\"title\":\"Test todo\""));
    assert!(json.contains("\"completed\":false"));
    // display_order is serialized as "order" due to serde rename
    assert!(json.contains("\"order\":1"));
    assert!(!json.contains("\"display_order\""));
}

#[wasm_bindgen_test]
fn test_todo_deserialization() {
    let json = r#"{"id":"x","title":"Hello","completed":true,"order":5}"#;
    let todo: Todo = serde_json::from_str(json).expect("deserialize");
    assert_eq!(todo.id, "x");
    assert_eq!(todo.title, "Hello");
    assert!(todo.completed);
    assert_eq!(todo.display_order, 5);
}

#[wasm_bindgen_test]
fn test_create_todo_serialization() {
    let input = CreateTodo {
        title: "New item".into(),
    };
    let json = serde_json::to_string(&input).expect("serialize");
    assert!(json.contains("\"title\":\"New item\""));
}

#[wasm_bindgen_test]
fn test_update_todo_skip_none_fields() {
    let input = UpdateTodo {
        title: Some("Updated".into()),
        completed: None,
        order: None,
    };
    let json = serde_json::to_string(&input).expect("serialize");
    assert!(json.contains("\"title\":\"Updated\""));
    // None fields should be skipped
    assert!(!json.contains("completed"));
    assert!(!json.contains("order"));
}

#[wasm_bindgen_test]
fn test_update_todo_all_fields() {
    let input = UpdateTodo {
        title: Some("New title".into()),
        completed: Some(true),
        order: Some(3),
    };
    let json = serde_json::to_string(&input).expect("serialize");
    assert!(json.contains("\"title\":\"New title\""));
    assert!(json.contains("\"completed\":true"));
    assert!(json.contains("\"order\":3"));
}

#[wasm_bindgen_test]
fn test_toggle_all_serialization() {
    let input = ToggleAll { completed: true };
    let json = serde_json::to_string(&input).expect("serialize");
    assert_eq!(json, r#"{"completed":true}"#);
}

// --- Filtering logic tests ---

fn sample_todos() -> Vec<Todo> {
    vec![
        Todo {
            id: "1".into(),
            title: "Active".into(),
            completed: false,
            display_order: 0,
        },
        Todo {
            id: "2".into(),
            title: "Done".into(),
            completed: true,
            display_order: 1,
        },
        Todo {
            id: "3".into(),
            title: "Also active".into(),
            completed: false,
            display_order: 2,
        },
    ]
}

fn apply_filter(todos: &[Todo], filter: Filter) -> Vec<&Todo> {
    todos
        .iter()
        .filter(|t| match filter {
            Filter::All => true,
            Filter::Active => !t.completed,
            Filter::Completed => t.completed,
        })
        .collect()
}

#[wasm_bindgen_test]
fn test_filter_all_returns_everything() {
    let todos = sample_todos();
    let filtered = apply_filter(&todos, Filter::All);
    assert_eq!(filtered.len(), 3);
}

#[wasm_bindgen_test]
fn test_filter_active_excludes_completed() {
    let todos = sample_todos();
    let filtered = apply_filter(&todos, Filter::Active);
    assert_eq!(filtered.len(), 2);
    assert!(filtered.iter().all(|t| !t.completed));
}

#[wasm_bindgen_test]
fn test_filter_completed_only_completed() {
    let todos = sample_todos();
    let filtered = apply_filter(&todos, Filter::Completed);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].title, "Done");
}

#[wasm_bindgen_test]
fn test_filter_empty_list() {
    let todos: Vec<Todo> = vec![];
    assert_eq!(apply_filter(&todos, Filter::All).len(), 0);
    assert_eq!(apply_filter(&todos, Filter::Active).len(), 0);
    assert_eq!(apply_filter(&todos, Filter::Completed).len(), 0);
}

#[wasm_bindgen_test]
fn test_active_count() {
    let todos = sample_todos();
    let active_count = todos.iter().filter(|t| !t.completed).count();
    assert_eq!(active_count, 2);
}

#[wasm_bindgen_test]
fn test_has_completed() {
    let todos = sample_todos();
    let has_completed = todos.iter().any(|t| t.completed);
    assert!(has_completed);

    let all_active: Vec<Todo> = vec![Todo {
        id: "1".into(),
        title: "Active".into(),
        completed: false,
        display_order: 0,
    }];
    assert!(!all_active.iter().any(|t| t.completed));
}

#[wasm_bindgen_test]
fn test_all_completed_check() {
    let todos = sample_todos();
    let all_completed = !todos.is_empty() && todos.iter().all(|t| t.completed);
    assert!(!all_completed);

    let all_done: Vec<Todo> = vec![Todo {
        id: "1".into(),
        title: "Done".into(),
        completed: true,
        display_order: 0,
    }];
    let all_completed = !all_done.is_empty() && all_done.iter().all(|t| t.completed);
    assert!(all_completed);
}
