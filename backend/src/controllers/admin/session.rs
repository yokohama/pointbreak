use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::middleware::{auth, error};

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<auth::AuthPayload>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let authorized = auth::authorize(&pool, payload, true).await?;
    Ok(Json(authorized))
}
