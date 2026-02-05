mod logic;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use axum_extra::headers::{authorization::Bearer, Authorization};
use axum_extra::TypedHeader;

use logic::auth;
use logic::auth::AuthError;

use logic::todo;
use logic::todo::Todo;

use std::sync::{Arc, RwLock};

type SharedState = Arc<RwLock<Vec<Todo>>>;

pub async fn run() {
    let initial_todos = todo::load_from_csv().unwrap_or_else(|err| {
        eprintln!("Warning: Could not load CSV ({}). Starting with empty list.", err);
        Vec::new()
    });
    let shared_state = Arc::new(RwLock::new(initial_todos));

    let app = Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(add_todo))
        // New login route to get a token
        .route("/login", post(auth::login))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Todo API running with JWT on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}


async fn get_todos(
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    State(state): State<SharedState>,
) -> Result<Json<Vec<Todo>>, AuthError> {
    // Validate the token
    auth::validate_jwt(auth_header)?;

    let todos = state.read().unwrap();
    Ok(Json(todos.clone()))
}

async fn add_todo(
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
