use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    middleware::{auth, error},
    models::user,
};

pub async fn index(
    _claims: auth::Claims, 
    State(pool): State<PgPool>
) -> Result<Json<impl Serialize>, error::AppError> {
    let users = user::all(&pool).await?;
    Ok(Json(users))
}
