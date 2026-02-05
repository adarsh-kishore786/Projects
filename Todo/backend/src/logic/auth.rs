use axum::Json;

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

use crate::logic::error::AuthError;

const JWT_SECRET: &[u8] = b"secret_key_change_me_in_production";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}

pub async fn login() -> Json<AuthBody> {
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

pub fn validate_jwt(header: Option<TypedHeader<Authorization<Bearer>>>) -> Result<Claims, AuthError> {
    let TypedHeader(Authorization(bearer)) = header.ok_or(AuthError::MissingToken)?;
    
    let token_data = decode::<Claims>(
        bearer.token(),
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )
    .map_err(|_| AuthError::InvalidToken)?;

    Ok(token_data.claims)
}
