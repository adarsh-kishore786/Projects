use axum::{
    Json, Router, extract::{State, Path}, routing::{get, post}
};
use sqlx::SqlitePool;

use crate::logic::auth::{self, Claims};
use crate::logic::todo;
use crate::logic::error::{AppError, ServerError};

use todo::Todo;

pub type SharedState = SqlitePool;

pub fn get_router(state: SharedState) -> Router {
    Router::new()
        .route("/login", post(auth::login))
        .route("/todos", get(get_todos))
        .route("/todos", post(add_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id/complete", post(complete_todo))
        .with_state(state)
}

async fn get_todos(
        _claims: Claims,
        State(pool): State<SharedState>,
    ) -> Result<Json<Vec<Todo>>, AppError> {

    let todos = todo::load_all(&pool).await.map_err(db_err)?;
    Ok(Json(todos))
}

async fn get_todo(
        _claims: Claims,
        Path(id): Path<u32>,
        State(pool): State<SharedState>,
    ) -> Result<Json<Todo>, AppError> {

    let todo = todo::find_by_id(&pool, id).await.map_err(db_err)?
        .ok_or_else(|| not_found_err(id))?;

    Ok(Json(todo))
}

#[derive(serde::Deserialize)]
struct CreateTodo {
    task: String,
}

async fn add_todo(
        _claims: Claims,
        State(pool): State<SharedState>,
        Json(input): Json<CreateTodo>,
    ) -> Result<Json<Todo>, AppError> {

    let todo = todo::create(&pool, &input.task).await.map_err(db_err)?;
    Ok(Json(todo))
}

async fn complete_todo(
        _claims: Claims,
        State(pool): State<SharedState>,
        Path(id): Path<u32>,
    ) -> Result<Json<Todo>, AppError> {

    let todo = todo::complete(&pool, id).await.map_err(db_err)?
        .ok_or_else(|| not_found_err(id))?;

    Ok(Json(todo))
}

fn db_err<E: std::fmt::Display>(err: E) -> ServerError {
    eprintln!("Database Error: {}", err);
    ServerError::Internal
}

fn not_found_err(id: u32) -> ServerError {
    eprintln!("Error: Could not find todo with id: {}", id);
    ServerError::NotFound
}