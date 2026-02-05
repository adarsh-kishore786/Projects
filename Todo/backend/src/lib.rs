mod logic;

use axum::{
    routing::{get, post},
    Router,
};

use logic::auth;
use logic::todo;
use logic::routes;

use std::sync::{Arc, RwLock};

pub async fn run() {
    let initial_todos = todo::load_from_csv().unwrap_or_else(|err| {
        eprintln!("Warning: Could not load CSV ({}). Starting with empty list.", err);
        Vec::new()
    });
    let shared_state = Arc::new(RwLock::new(initial_todos));

    let app = Router::new()
        .route("/login", post(auth::login))

        .route("/todos", get(routes::get_todos))
        .route("/todos", post(routes::add_todo))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Todo API running with JWT on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
