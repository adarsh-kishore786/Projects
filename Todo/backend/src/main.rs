use axum::{extract::State, routing::{get, post}, Json, Router};
use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::fs::OpenOptions;
use std::io::ErrorKind;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

type SharedState = Arc<RwLock<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    // 1. Load existing data from CSV
    let initial_todos = load_from_csv().unwrap_or_else(|err| {
        eprintln!("Note: Could not load CSV ({}). Starting fresh.", err);
        Vec::new()
    });

    // 2. Put that data into our Shared State
    let shared_state = Arc::new(RwLock::new(initial_todos));

    let app = Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(add_todo))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Todo API running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// --- NEW: Loading Logic ---
fn load_from_csv() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    let file = match OpenOptions::new().read(true).open("todos.csv") {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };

    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);

    let mut list = Vec::new();

    for result in rdr.deserialize() {
        let record: Todo = result?;
        list.push(record);
    }
    Ok(list)
}

async fn get_todos(State(state): State<SharedState>) -> Json<Vec<Todo>> {
    let todos = state.read().unwrap();
    Json(todos.clone())
}

async fn add_todo(State(state): State<SharedState>, Json(input): Json<Todo>) -> Json<Todo> {
    let mut todos = state.write().unwrap();
    todos.push(input.clone());
    save_to_csv(&input).expect("Failed to write to CSV");
    Json(input)
}

fn save_to_csv(todo: &Todo) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("todos.csv")?;
    
    let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.serialize(todo)?;
    wtr.flush()?;
    Ok(())
}
