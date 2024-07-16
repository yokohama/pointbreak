use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    midleware::{auth, error},
    models::user,
};

pub async fn authorization(
    State(pool): State<PgPool>,
    Json(payload): Json<auth::AuthPayload>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let authorized = auth::authorize(&pool, payload, true).await?;
    Ok(Json(authorized))
}

pub async fn dashboard(
    claims: auth::Claims, 
    State(pool): State<PgPool>
) -> Result<Json<impl Serialize>, error::AppError> {
    let current_user = claims.get_current_user(&pool).await?;
    Ok(Json(current_user))
}

pub async fn users(
    _claims: auth::Claims, 
    State(pool): State<PgPool>
) -> Result<Json<impl Serialize>, error::AppError> {
    let users = user::all(&pool).await?;
    Ok(Json(users))
}
