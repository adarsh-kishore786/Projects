mod logic;

use logic::auth::AuthError;

use logic::todo;
use logic::todo::Todo;

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

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
        .route("/login", post(login))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Todo API running with JWT on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
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
    if let Err(e) = todo::save_to_csv(&input) {
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
