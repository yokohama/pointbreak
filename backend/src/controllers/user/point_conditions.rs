use axum::{ 
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    midleware::{auth, error},
    models::point_condition,
};

#[derive(serde::Deserialize)]
pub struct NewRequest {
    lat: f64,
    lon: f64,
}

pub async fn index(
    State(pool): State<PgPool>,
    claims: auth::Claims,
) -> Result<Json<impl Serialize>, error::AppError> {
    let current_user = claims.get_current_user(&pool).await?;
    let point_conditions: Vec<_> = point_condition::find_by_user_id(
        &pool, 
        current_user.id
    ).await?;
    Ok(Json(point_conditions))
}

pub async fn create(
    State(pool): State<PgPool>,
    claims: auth::Claims,
    Json(payload): Json<NewRequest>,
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
