use axum::{ 
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    midleware::error,
    models::user,
    requests,
};

pub async fn create(
    State(pool): State<PgPool>,
    Json(payload): Json<requests::user::NewRegistration>,
) -> Result<Json<impl Serialize>, error::AppError> {
    let new_user = user::New {
        email: payload.email, 
        password: payload.password,
    };
    let create_user = user::create(&pool, new_user).await?;
    Ok(Json(create_user))
}
