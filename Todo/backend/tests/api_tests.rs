use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use serde_json::{json, Value};
use tower::ServiceExt; // for `oneshot`
use sqlx::sqlite::SqlitePoolOptions;
use backend::logic::router;
use backend::logic::todo;

async fn setup_app() -> axum::Router {
    // 1. Initialize an in-memory database for testing
    let pool = SqlitePoolOptions::new()
        .connect("sqlite::memory:")
        .await
        .expect("Failed to connect to in-memory database");

    todo::init_db(&pool).await.expect("Failed to initialize database tables");

    // 2. Create the router with the test pool
    router::get_router(pool)
}

async fn get_token(app: &axum::Router, username: &str) -> String {
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/signup")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"username": username, "password": "password"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    body["access_token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_signup_and_login() {
    let app = setup_app().await;

    // 1. Test Signup
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/signup")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert!(body["access_token"].is_string());

    // 2. Test Login
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/login")
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({
                        "username": "testuser",
                        "password": "password123"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_project_crud() {
    let app = setup_app().await;
    let token = get_token(&app, "user1").await;
    let auth_header = format!("Bearer {}", token);

    // 1. Create Project
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/projects")
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({"name": "Work", "color": "Blue"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let project: Value = serde_json::from_slice(&body).unwrap();
    let project_id = project["id"].as_i64().unwrap();

    // 2. Update Project (PATCH)
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::PATCH)
                .uri(format!("/projects/{}", project_id))
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(
                    json!({"name": "Work Updated"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    // 3. Delete Project
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/projects/{}", project_id))
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_task_crud() {
    let app = setup_app().await;
    let token = get_token(&app, "user2").await;
    let auth_header = format!("Bearer {}", token);

    // 1. Create Project
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/projects")
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"name": "Inbox"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let project_id = serde_json::from_slice::<Value>(&body).unwrap()["id"].as_i64().unwrap();

    // 2. Create Task
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(format!("/projects/{}/tasks", project_id))
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"title": "Fix API tests"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let task_id = serde_json::from_slice::<Value>(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()["id"].as_i64().unwrap();

    // 3. Complete Task
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(format!("/tasks/{}/complete", task_id))
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_comment_crud() {
    let app = setup_app().await;
    let token = get_token(&app, "user3").await;
    let auth_header = format!("Bearer {}", token);

    // 1. Setup Project & Task
    let response = app.clone().oneshot(Request::builder().method(http::Method::POST).uri("/projects").header(http::header::AUTHORIZATION, &auth_header).header(http::header::CONTENT_TYPE, "application/json").body(Body::from(json!({"name": "P"}).to_string())).unwrap()).await.unwrap();
    let project_id = serde_json::from_slice::<Value>(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()["id"].as_i64().unwrap();

    let response = app.clone().oneshot(Request::builder().method(http::Method::POST).uri(format!("/projects/{}/tasks", project_id)).header(http::header::AUTHORIZATION, &auth_header).header(http::header::CONTENT_TYPE, "application/json").body(Body::from(json!({"title": "T"}).to_string())).unwrap()).await.unwrap();
    let task_id = serde_json::from_slice::<Value>(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()["id"].as_i64().unwrap();

    // 2. Add Comment
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri(format!("/tasks/{}/comments", task_id))
                .header(http::header::AUTHORIZATION, &auth_header)
                .header(http::header::CONTENT_TYPE, "application/json")
                .body(Body::from(json!({"content": "C1"}).to_string()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let comment_id = serde_json::from_slice::<Value>(&response.into_body().collect().await.unwrap().to_bytes()).unwrap()["id"].as_i64().unwrap();

    // 3. Delete Comment
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(format!("/comments/{}", comment_id))
                .header(http::header::AUTHORIZATION, &auth_header)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_error_cases() {
    let app = setup_app().await;

    // 1. Unauthorized
    let response = app.clone().oneshot(Request::builder().uri("/projects").body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

    // 2. Not Found
    let token = get_token(&app, "user4").await;
    let auth_header = format!("Bearer {}", token);
    let response = app.oneshot(Request::builder().uri("/projects/999").header(http::header::AUTHORIZATION, &auth_header).body(Body::empty()).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
