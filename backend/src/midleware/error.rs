use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use axum::{
    http::request::Parts,
    extract::OriginalUri,
};

use serde_json::json;

use tracing::info;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ConflictError(String),
    UnAuthorized(String),
    MissingCredentials(String),
    DatabaseError(String),
    InternalServerError(String),
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => AppError::DatabaseError(error.to_string())
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, err_msg) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),

            AppError::ConflictError(msg) => (StatusCode::CONFLICT, msg),
            AppError::UnAuthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::MissingCredentials(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::InternalServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "status": status.as_u16(),
            "error": err_msg,
        }));

        body.into_response()
    }
}

pub async fn not_found(OriginalUri(uri): OriginalUri, parts: Parts) -> impl IntoResponse {
    info!("{} : [{}] - {}", 
        StatusCode::NOT_FOUND,
        parts.method,
        uri
    );
    AppError::NotFound(format!("{} is not found.", uri))
}
