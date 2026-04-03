use axum::body::Body;
use axum::http::{Request, StatusCode};
use backend::db;
use backend::models::Todo;
use backend::routes::api_router;
use http_body_util::BodyExt;
use tower::ServiceExt;

fn setup_app() -> axum::Router {
    std::env::set_var("DATABASE_URL", ":memory:");
    let pool = db::init_db().expect("init test db");
    api_router(pool)
}

async fn body_json<T: serde::de::DeserializeOwned>(body: Body) -> T {
    let bytes = body.collect().await.expect("collect body").to_bytes();
    serde_json::from_slice(&bytes).expect("parse json")
}

async fn create_todo(app: &axum::Router, title: &str) -> (StatusCode, Option<Todo>) {
    let req = Request::builder()
        .method("POST")
        .uri("/api/todos")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::json!({"title": title}).to_string()))
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    let status = resp.status();
    if status == StatusCode::CREATED {
        let todo: Todo = body_json(resp.into_body()).await;
        (status, Some(todo))
    } else {
        (status, None)
    }
}

// --- List ---

#[tokio::test]
async fn test_list_empty() {
    let app = setup_app();
    let req = Request::builder()
        .uri("/api/todos")
        .body(Body::empty())
        .expect("build request");

    let resp = app.oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::OK);

    let todos: Vec<Todo> = body_json(resp.into_body()).await;
    assert!(todos.is_empty());
}

#[tokio::test]
async fn test_list_after_create() {
    let app = setup_app();
    create_todo(&app, "Item 1").await;
    create_todo(&app, "Item 2").await;

    let req = Request::builder()
        .uri("/api/todos")
        .body(Body::empty())
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::OK);

    let todos: Vec<Todo> = body_json(resp.into_body()).await;
    assert_eq!(todos.len(), 2);
}

// --- Create ---

#[tokio::test]
async fn test_create_todo() {
    let app = setup_app();
    let (status, todo) = create_todo(&app, "Buy milk").await;
    assert_eq!(status, StatusCode::CREATED);

    let todo = todo.expect("should have todo");
    assert_eq!(todo.title, "Buy milk");
    assert!(!todo.completed);
    assert!(!todo.id.is_empty());
}

#[tokio::test]
async fn test_create_empty_title() {
    let app = setup_app();
    let (status, _) = create_todo(&app, "").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_create_whitespace_title() {
    let app = setup_app();
    let (status, _) = create_todo(&app, "   ").await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// --- Get by ID ---

#[tokio::test]
async fn test_get_todo() {
    let app = setup_app();
    let (_, todo) = create_todo(&app, "Find me").await;
    let todo = todo.expect("created");

    let req = Request::builder()
        .uri(format!("/api/todos/{}", todo.id))
        .body(Body::empty())
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::OK);

    let found: Todo = body_json(resp.into_body()).await;
    assert_eq!(found.id, todo.id);
    assert_eq!(found.title, "Find me");
}

#[tokio::test]
async fn test_get_nonexistent() {
    let app = setup_app();
    let req = Request::builder()
        .uri("/api/todos/nonexistent-id")
        .body(Body::empty())
        .expect("build request");

    let resp = app.oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// --- Update ---

#[tokio::test]
async fn test_update_title() {
    let app = setup_app();
    let (_, todo) = create_todo(&app, "Original").await;
    let todo = todo.expect("created");

    let req = Request::builder()
        .method("PATCH")
        .uri(format!("/api/todos/{}", todo.id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"title": "Updated"}).to_string(),
        ))
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::OK);

    let updated: Todo = body_json(resp.into_body()).await;
    assert_eq!(updated.title, "Updated");
    assert!(!updated.completed);
}

#[tokio::test]
async fn test_update_completed() {
    let app = setup_app();
    let (_, todo) = create_todo(&app, "Toggle me").await;
    let todo = todo.expect("created");

    let req = Request::builder()
        .method("PATCH")
        .uri(format!("/api/todos/{}", todo.id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"completed": true}).to_string(),
        ))
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::OK);

    let updated: Todo = body_json(resp.into_body()).await;
    assert!(updated.completed);
}

#[tokio::test]
async fn test_update_nonexistent() {
    let app = setup_app();
    let req = Request::builder()
        .method("PATCH")
        .uri("/api/todos/nonexistent-id")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::json!({"title": "Nope"}).to_string()))
        .expect("build request");

    let resp = app.oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// --- Delete ---

#[tokio::test]
async fn test_delete_todo() {
    let app = setup_app();
    let (_, todo) = create_todo(&app, "Delete me").await;
    let todo = todo.expect("created");

    let req = Request::builder()
        .method("DELETE")
        .uri(format!("/api/todos/{}", todo.id))
        .body(Body::empty())
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify it's gone
    let req = Request::builder()
        .uri(format!("/api/todos/{}", todo.id))
        .body(Body::empty())
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_delete_nonexistent() {
    let app = setup_app();
    let req = Request::builder()
        .method("DELETE")
        .uri("/api/todos/nonexistent-id")
        .body(Body::empty())
        .expect("build request");

    let resp = app.oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}

// --- Delete completed ---

#[tokio::test]
async fn test_delete_completed() {
    let app = setup_app();

    // Create two todos, mark one completed
    let (_, todo1) = create_todo(&app, "Keep").await;
    let _ = todo1.expect("created");
    let (_, todo2) = create_todo(&app, "Done").await;
    let todo2 = todo2.expect("created");

    // Mark todo2 as completed
    let req = Request::builder()
        .method("PATCH")
        .uri(format!("/api/todos/{}", todo2.id))
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"completed": true}).to_string(),
        ))
        .expect("build request");
    app.clone().oneshot(req).await.expect("request");

    // Delete completed
    let req = Request::builder()
        .method("DELETE")
        .uri("/api/todos/completed")
        .body(Body::empty())
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::NO_CONTENT);

    // Verify only 1 remains
    let req = Request::builder()
        .uri("/api/todos")
        .body(Body::empty())
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    let todos: Vec<Todo> = body_json(resp.into_body()).await;
    assert_eq!(todos.len(), 1);
    assert_eq!(todos[0].title, "Keep");
}

// --- Toggle all ---

#[tokio::test]
async fn test_toggle_all() {
    let app = setup_app();
    create_todo(&app, "A").await;
    create_todo(&app, "B").await;

    // Toggle all to completed
    let req = Request::builder()
        .method("PATCH")
        .uri("/api/todos/toggle-all")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"completed": true}).to_string(),
        ))
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    assert_eq!(resp.status(), StatusCode::OK);

    let todos: Vec<Todo> = body_json(resp.into_body()).await;
    assert_eq!(todos.len(), 2);
    assert!(todos.iter().all(|t| t.completed));

    // Toggle all back to active
    let req = Request::builder()
        .method("PATCH")
        .uri("/api/todos/toggle-all")
        .header("content-type", "application/json")
        .body(Body::from(
            serde_json::json!({"completed": false}).to_string(),
        ))
        .expect("build request");

    let resp = app.clone().oneshot(req).await.expect("request");
    let todos: Vec<Todo> = body_json(resp.into_body()).await;
    assert!(todos.iter().all(|t| !t.completed));
}
