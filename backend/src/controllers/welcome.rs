use axum::{
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::midleware::error;

pub async fn show(State(pool): State<PgPool>) -> Result<Json<impl Serialize>, error::AppError> {
    println!("{:#?}", pool);

    Ok(Json("Hello World!"))
}
