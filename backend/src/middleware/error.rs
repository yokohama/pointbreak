use axum::extract::rejection::JsonRejection;
use reqwest;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use axum::{
    http::request::Parts,
    extract::OriginalUri,
};

use serde_json::json;
use tracing::{info, error};

use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),

    #[error("Conflict occurred: {0}")]
    ConflictError(String),

    #[error("Unauthorized access attempt: {0}")]
    UnAuthorized(String),

    #[error("Missing credentials: {0}")]
    MissingCredentials(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Form input error: {0}")]
    FormInputError(String),
}

impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        match error {
            _ => {
                error!("{:#?}", error);
                AppError::InternalServerError(error.to_string())
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => {
                error!("{:#?}", error);
                AppError::DatabaseError(error.to_string())
            }
        }
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> Self {
        AppError::ValidationError(error.to_string())
    }
}

impl From<JsonRejection> for AppError {
    fn from(error: JsonRejection) -> Self {
        AppError::FormInputError(format!("Json error: {:?}", error))
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
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::FormInputError(msg) => (StatusCode::BAD_REQUEST, msg),
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
