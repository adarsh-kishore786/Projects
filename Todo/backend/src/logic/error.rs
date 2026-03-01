use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AuthError {
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

pub enum ServerError {
    NotFound(&'static str),
    ParameterError(&'static str),
    Internal,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::NotFound(resource) => {
                (StatusCode::NOT_FOUND, format!("{} not found", resource)).into_response()
            }
            ServerError::ParameterError(msg) => {
                (StatusCode::BAD_REQUEST, msg.to_string()).into_response()
            }
            ServerError::Internal => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

// Custom Extractors to prevent information leakage

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use async_trait::async_trait;

pub struct Path<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    T: serde::de::DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                eprintln!("Path Rejection: {}", rejection); // Log for developers
                Err(ServerError::ParameterError("Invalid path parameter").into())
            }
        }
    }
}

pub struct Json<T>(pub T);

#[async_trait]
impl<S, T> axum::extract::FromRequest<S> for Json<T>
where
    T: serde::de::DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: axum::http::Request<axum::body::Body>, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Json::<T>::from_request(req, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                eprintln!("JSON Rejection: {}", rejection); // Log for YOU
                Err(ServerError::ParameterError("Invalid JSON payload").into())
            }
        }
    }
}

impl<T> IntoResponse for Json<T>
where
    T: serde::Serialize,
{
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

pub enum AppError {
    Auth(AuthError),
    Server(ServerError),
}

impl From<AuthError> for AppError {
    fn from(inner: AuthError) -> Self {
        AppError::Auth(inner)
    }
}

impl From<ServerError> for AppError {
    fn from(inner: ServerError) -> Self {
        AppError::Server(inner)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Auth(err) => err.into_response(),
            AppError::Server(err) => err.into_response(),
        }
    }
}
