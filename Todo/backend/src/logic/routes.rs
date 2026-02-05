use axum::{
    extract::State,
    Json
};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader
};

use std::sync::{Arc, RwLock};

use crate::logic::auth;
use crate::logic::todo;

use auth::AuthError;
use todo::Todo;

type SharedState = Arc<RwLock<Vec<Todo>>>;

pub async fn get_todos(
        auth_header: Option<TypedHeader<Authorization<Bearer>>>,
        State(state): State<SharedState>,
    ) -> Result<Json<Vec<Todo>>, AuthError> {

    // Validate the token
    auth::validate_jwt(auth_header)?;

    let todos = state.read().unwrap();
    Ok(Json(todos.clone()))
}

pub async fn add_todo(
        auth_header: Option<TypedHeader<Authorization<Bearer>>>,
        State(state): State<SharedState>,
        Json(input): Json<Todo>,
    ) -> Result<Json<Todo>, AuthError> {

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
        eprintln!("CRITICAL: Failed to save to CSV: {}", e);
        // Optional: You could return a 500 Internal Server Error here if you prefer
    }

    Ok(Json(input))
}
