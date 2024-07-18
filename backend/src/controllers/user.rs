use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    midleware::{auth, error},
    models::{user, point_condition},
};

#[derive(serde::Deserialize)]
pub struct NewUserRequest {
    email: String,
    password: String,
}

pub async fn registration(
    State(pool): State<PgPool>,
    Json(payload): Json<NewUserRequest>,
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

#[derive(serde::Deserialize)]
pub struct NewPointConditionRequest {
    lat: f64,
    lon: f64,
}

pub async fn create_point_conditions(
    State(pool): State<PgPool>,
    claims: auth::Claims,
    Json(payload): Json<NewPointConditionRequest>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let current_user = claims.get_current_user(&pool).await?;
    let new = point_condition::New { 
        user_id: current_user.id, 
        lat: payload.lat,
        lon: payload.lon,
    };
    let created = point_condition::create(&pool, new).await?;
    Ok(Json(created))
}

pub async fn point_conditions(
    State(pool): State<PgPool>,
    claims: auth::Claims,
) -> Result<Json<impl Serialize>, error::AppError> {
    let current_user = claims.get_current_user(&pool).await?;
    let point_conditions: Vec<point_condition::Entry> = point_condition::find_by_user_id(&pool, current_user.id).await?;
    Ok(Json(point_conditions))
}
