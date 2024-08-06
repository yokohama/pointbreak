use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::middleware::{auth, error};

pub async fn show(
    claims: auth::Claims, 
    State(pool): State<PgPool>
) -> Result<Json<impl Serialize>, error::AppError> {
    let current_user = claims.get_current_user(&pool).await?;
    Ok(Json(current_user))
}
