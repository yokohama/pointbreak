use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::middleware::error;
use crate::models::point_condition;

pub async fn show(State(pool): State<PgPool>) -> Result<Json<impl Serialize>, error::AppError> {
    println!("{:#?}", pool);

    let conditions: Vec<point_condition::Entry> = point_condition::all(&pool)
        .await?;

    Ok(Json(conditions))
}
