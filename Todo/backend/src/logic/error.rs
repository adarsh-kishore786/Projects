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
    NotFound,
    Internal,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let (status, msg) = match self {
            ServerError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            ServerError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };
        (status, msg).into_response()
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
