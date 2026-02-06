use axum::{
    Json, Router, extract::{State, Path}, routing::{get, post}
};

use std::sync::{Arc, RwLock};

use crate::logic::auth::{self, Claims};
use crate::logic::todo;
use crate::logic::error::{AppError, ServerError};

use todo::Todo;

pub type SharedState = Arc<RwLock<Vec<Todo>>>;

pub fn initialize_state(todos: Vec<Todo>) -> SharedState {
    Arc::new(RwLock::new(todos))
}

pub fn get_router(state: SharedState) -> Router {
    return Router::new()
        .route("/login", post(auth::login))
        .route("/todos", get(get_todos))
        .route("/todos", post(add_todo))
        .route("/todos/:id", get(get_todo))
        .route("/todos/:id/complete", post(complete_todo))
        .with_state(state);
}

async fn get_todos(
        _claims: Claims,
        State(state): State<SharedState>,
    ) -> Result<Json<Vec<Todo>>, AppError> {

    let todos = match state.read() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error: Failed to fetch the todos from server: {}", err);
            return Err(ServerError::Internal.into());
        }
    };

    Ok(Json(todos.clone()))
}

async fn get_todo(
        _claims: Claims,
        Path(id): Path<u32>,
        State(state): State<SharedState>,
    ) -> Result<Json<Todo>, AppError> {

    let todos = match state.read() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error: Failed to fetch the todos from server: {}", err);
            return Err(ServerError::Internal.into());
        }
    };

    for todo in todos.iter() {
        if todo.id == id {
            return Ok(Json(todo.clone()));
        }
    }

    eprintln!("Error: No todo exists with id {}", id);
    return Err(ServerError::NotFound.into());
}

async fn add_todo(
        _claims: Claims,
        State(state): State<SharedState>,
        Json(input): Json<Todo>,
    ) -> Result<Json<Todo>, AppError> {

    // 1. Update RAM (State)
    {
        let mut todos = state.write().unwrap();
        todos.push(input.clone());
    } // Lock is dropped here

    // 2. Update Disk (CSV)
    save_or_error(&input)?;

    Ok(Json(input))
}

async fn complete_todo(
        _claims: Claims,
        State(state): State<SharedState>,
        Path(id): Path<u32>,
    ) -> Result<Json<Todo>, AppError> {

    let mut todos = match state.write() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error: Failed to fetch the todos from server: {}", err);
            return Err(ServerError::Internal.into());
        }
    };

    for todo in todos.iter_mut() {
        if todo.id == id {
            todo.completed = true;
            save_or_error(todo)?;
            return Ok(Json(todo.clone()));
        }
    }

    eprint!("Error: Could not find todo with id: {}", id);
    return Err(ServerError::NotFound.into());
}

fn save_or_error(todo: &Todo) -> Result<(), ServerError> {
    if let Err(e) = todo::save_to_csv(todo) {
        eprintln!("Error: Failed to save to CSV: {}", e);
        return Err(ServerError::Internal.into());
    }
    Ok(())
}
