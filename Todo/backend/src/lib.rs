mod logic;

use logic::todo;
use logic::router;

pub async fn run() {
    let initial_todos = get_todos();
    let shared_state = router::initialize_state(initial_todos);
    let app = router::get_router(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Todo API running with JWT on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

fn get_todos() -> Vec<todo::Todo> {
    return todo::load_from_csv().unwrap_or_else(|err| {
        eprintln!("Warning: Could not load CSV ({}). Starting with empty list.", err);
        return Vec::new();
    });
}
