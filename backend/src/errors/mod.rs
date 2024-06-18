use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    http::request::Parts,
    Json,
    extract::OriginalUri,
};
use serde_json::json;
use std::fmt;

use tracing::info;

#[derive(Debug)]
pub enum AppError {
    WrongCredentials(String),
    TokenCreation(String),
    InvalidToken(String),
    InternalServerError(String),
    NotFound(String),
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
            AppError::WrongCredentials(msg) 
                => (StatusCode::UNAUTHORIZED, msg),
            AppError::TokenCreation(msg) 
                => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InvalidToken(msg) 
                => (StatusCode::BAD_REQUEST, msg),
            AppError::InternalServerError(msg) 
                => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::NotFound(msg) 
                => (StatusCode::NOT_FOUND, msg),
        };

        let body = Json(json!({
            "status": status.as_u16(),
            "error": error_message,
        }));

        body.into_response()
    }
}

pub async fn handler_404(OriginalUri(uri): OriginalUri, parts: Parts) -> impl IntoResponse {
    info!("{} : [{}] - {}", 
        StatusCode::NOT_FOUND.as_u16(),
        parts.method,
        uri
    );
    AppError::NotFound(format!("{} is not found.", uri))
}
