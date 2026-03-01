use axum::{
    extract::{FromRequestParts, State},
    http::request::Parts,
    async_trait
};

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

use jsonwebtoken::{
    encode, decode, 
    DecodingKey, EncodingKey,
    Header, Validation
};

use serde::{Serialize,Deserialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::SqlitePool;

use crate::logic::error::{AuthError, AppError, ServerError, Json};
use crate::logic::todo;

const JWT_SECRET: &[u8] = b"secret_key_change_me_in_production";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // This will store the numeric User ID as a string
    pub exp: usize,
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
            .await
            .map_err(|_| AuthError::MissingToken)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(JWT_SECRET),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[derive(Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

#[derive(Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String,
}

pub async fn signup(
    State(pool): State<SqlitePool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AppError> {

    let password_hash = hash(payload.password, DEFAULT_COST).map_err(|_| ServerError::Internal)?;
    
    let user_id = todo::create_user(&pool, &payload.username, &password_hash)
        .await
        .map_err(|err| {
            eprintln!("Error: {}", err);
            ServerError::Internal
        })?;

    let token = create_jwt(user_id)?;

    Ok(Json(AuthBody {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

pub async fn login(
    State(pool): State<SqlitePool>,
    Json(payload): Json<AuthPayload>,
) -> Result<Json<AuthBody>, AppError> {
    let user = todo::find_user_by_username(&pool, &payload.username)
        .await
        .map_err(|_| ServerError::Internal)?
        .ok_or(AuthError::InvalidToken)?;

    let is_valid = verify(payload.password, &user.password_hash).map_err(|_| ServerError::Internal)?;

    if !is_valid {
        return Err(AuthError::InvalidToken.into());
    }

    let token = create_jwt(user.id)?;

    Ok(Json(AuthBody {
        access_token: token,
        token_type: "Bearer".to_string(),
    }))
}

fn create_jwt(user_id: i64) -> Result<String, ServerError> {
    let claims = Claims {
        sub: user_id.to_string(),
        exp: 2000000000, 
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|_| ServerError::Internal)
}
