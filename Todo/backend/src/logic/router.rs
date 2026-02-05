use axum::{
    Json, Router, extract::State, routing::{get, post}
};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader
};

use std::sync::{Arc, RwLock};

use crate::logic::auth;
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
        .with_state(state);
}

async fn get_todos(
        auth_header: Option<TypedHeader<Authorization<Bearer>>>,
        State(state): State<SharedState>,
    ) -> Result<Json<Vec<Todo>>, AppError> {

    // Validate the token
    auth::validate_jwt(auth_header)?;

    let todos = match state.read() {
        Ok(t) => t,
        Err(err) => {
            eprintln!("Error: Failed to fetch the todos from server: {}", err);
            return Err(ServerError::Internal.into());
        }
    };

    Ok(Json(todos.clone()))
}

async fn add_todo(
        auth_header: Option<TypedHeader<Authorization<Bearer>>>,
        State(state): State<SharedState>,
        Json(input): Json<Todo>,
    ) -> Result<Json<Todo>, AppError> {

    // 1. Check JWT first
    auth::validate_jwt(auth_header)?;

    // 2. Update RAM (State)
    {
        let mut todos = state.write().unwrap();
        todos.push(input.clone());
    } // Lock is dropped here

    // 3. Update Disk (CSV)
    // We use 'if let Err' to catch errors without breaking the function return type
    if let Err(e) = todo::save_to_csv(&input) {
        eprintln!("Error: Failed to save to CSV: {}", e);
        return Err(ServerError::Internal.into());
    }

    Ok(Json(input))
}
