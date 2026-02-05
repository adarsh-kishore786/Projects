mod logic;

use logic::todo;
use logic::router;

use std::sync::{Arc, RwLock};

pub async fn run() {
    let initial_todos = todo::load_from_csv().unwrap_or_else(|err| {
        eprintln!("Warning: Could not load CSV ({}). Starting with empty list.", err);
        return Vec::new();
    });
    let shared_state = Arc::new(RwLock::new(initial_todos));

    let app = router::get_router(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Todo API running with JWT on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
