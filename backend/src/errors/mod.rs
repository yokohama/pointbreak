use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    WrongCredentials(String),
    MissingCredentials(String),
    TokenCreation(String),
    InvalidToken(String),
    InternalServerError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::WrongCredentials(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::MissingCredentials(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::TokenCreation(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InvalidToken(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "status": status.as_u16(),
            "error": error_message,
        }));

        body.into_response()
    }
}
