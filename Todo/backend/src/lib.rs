mod logic;

use logic::todo;
use logic::router;
use sqlx::sqlite::SqlitePoolOptions;

pub async fn run() {
    // 1. Initialize Database
    let database_url = "sqlite:todos.db?mode=rwc";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Failed to connect to SQLite");

    todo::init_db(&pool).await.expect("Failed to initialize database table");

    // 2. Start Router
    let app = router::get_router(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Todo API running with SQLx (SQLite) on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}