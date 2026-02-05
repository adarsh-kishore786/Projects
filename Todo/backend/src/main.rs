use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
    response::{IntoResponse, Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use std::fs::OpenOptions;
use std::io::ErrorKind;
use csv::{ReaderBuilder, WriterBuilder};

// --- AUTH UTILS ---
const JWT_SECRET: &[u8] = b"secret_key_change_me_in_production";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // Subject (user identifier)
    exp: usize,  // Expiration time
}

#[derive(Serialize)]
struct AuthBody {
    access_token: String,
    token_type: String,
}

// Custom error for auth failures
enum AuthError {
    MissingToken,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            AuthError::InvalidToken => (StatusCode::FORBIDDEN, "Invalid token"),
        };
        (status, msg).into_response()
    }
}

// --- TODO LOGIC (Existing) ---
#[derive(Serialize, Deserialize, Clone, Debug)]
struct Todo {
    id: u32,
    task: String,
    completed: bool,
}

type SharedState = Arc<RwLock<Vec<Todo>>>;

#[tokio::main]
async fn main() {
    let initial_todos = load_from_csv().unwrap_or_else(|err| {
        eprintln!("Warning: Could not load CSV ({}). Starting with empty list.", err);
        Vec::new()
    });
    let shared_state = Arc::new(RwLock::new(initial_todos));

    let app = Router::new()
        .route("/todos", get(get_todos))
        .route("/todos", post(add_todo))
        // New login route to get a token
        .route("/login", post(login))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Todo API running with JWT on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// --- HANDLERS ---
fn load_from_csv() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    // Open the file. If it doesn't exist, just return an empty list 
    // instead of an error, because a fresh app won't have a file yet.
    let file = match OpenOptions::new().read(true).open("todos.csv") {
        Ok(file) => file,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Vec::new()),
        Err(e) => return Err(e.into()),
    };

    let mut rdr = ReaderBuilder::new()
        .has_headers(false) // Set to true if your CSV has "id,task,completed" as the first line
        .from_reader(file);

    let mut list = Vec::new();

    for result in rdr.deserialize() {
        let record: Todo = result?;
        list.push(record);
    }
    
    println!("Successfully loaded {} todos from disk.", list.len());
    Ok(list)
}

// --- RESTORED & IMPROVED: Saving Logic ---
fn save_to_csv(todo: &Todo) -> Result<(), Box<dyn std::error::Error>> {
    let file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // Create the file if it doesn't exist
        .open("todos.csv")?;
    
    let mut wtr = WriterBuilder::new()
        .has_headers(false)
        .from_writer(file);
        
    wtr.serialize(todo)?;
    wtr.flush()?; // Ensure data is physically written to disk
    Ok(())
}

// Dummy Login: In a real app, you'd verify a username/password here
async fn login() -> Json<AuthBody> {
    let claims = Claims {
        sub: "user_123".to_owned(),
        exp: 2000000000, // Year 2033 (use actual timestamps in production!)
    };

    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET)).unwrap();

    Json(AuthBody {
        access_token: token,
        token_type: "Bearer".to_string(),
    })
}

// Protected route: We add the TypedHeader as an argument
async fn get_todos(
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    State(state): State<SharedState>,
) -> Result<Json<Vec<Todo>>, AuthError> {
    // Validate the token
    validate_jwt(auth_header)?;

    let todos = state.read().unwrap();
    Ok(Json(todos.clone()))
}

async fn add_todo(
    auth_header: Option<TypedHeader<Authorization<Bearer>>>,
    State(state): State<SharedState>,
    Json(input): Json<Todo>,
) -> Result<Json<Todo>, AuthError> {
    // 1. Check JWT first
    validate_jwt(auth_header)?;

    // 2. Update RAM (State)
    {
        let mut todos = state.write().unwrap();
        todos.push(input.clone());
    } // Lock is dropped here

    // 3. Update Disk (CSV)
    // We use 'if let Err' to catch errors without breaking the function return type
    if let Err(e) = save_to_csv(&input) {
        eprintln!("CRITICAL: Failed to save to CSV: {}", e);
        // Optional: You could return a 500 Internal Server Error here if you prefer
    }

    Ok(Json(input))
}

// --- HELPER ---
fn validate_jwt(header: Option<TypedHeader<Authorization<Bearer>>>) -> Result<Claims, AuthError> {
    let TypedHeader(Authorization(bearer)) = header.ok_or(AuthError::MissingToken)?;
    
    let token_data = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    Ok(token_data.claims)
}
