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

#[derive(serde::Deserialize)]
pub struct CreateUserRequest {
    email: String,
    password: String,
}

pub async fn registration(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let new_user = user::NewUser {
        email: payload.email, 
        password: payload.password,
    };
    let create_user = user::create(&pool, new_user).await?;
    Ok(Json(create_user))
}

pub async fn authorization(
    State(pool): State<PgPool>,
    Json(payload): Json<auth::AuthPayload>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let authorized = auth::authorize(&pool, payload, false).await?;
    Ok(Json(authorized))
}

pub async fn dashboard(
    State(pool): State<PgPool>,
    claims: auth::Claims,
) -> Result<Json<impl Serialize>, error::AppError> {
    let current_user = claims.get_current_user(&pool).await?;
    Ok(Json(current_user))
}
