use serde::Serialize;
use sqlx::{
    PgPool,
    query_as,
    FromRow,
};

use tracing::error;

use crate::midleware::error;

#[derive(Serialize)]
pub struct New {
    pub user_id: i32,
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Created {
    id: i32,
    user_id: i32,
    lat: f64,
    lon: f64,
}

#[derive(FromRow, Serialize)]
pub struct Entry {
    id: i32,
    user_id: i32,
    lat: f64,
    lon: f64,
}

pub async fn create(
    pool: &PgPool, 
    new_env: New
) -> Result<Created, error::AppError> {
    let sql = r#"
        INSERT INTO point_conditions (
            user_id, 
            lat,
            lon,
            created_at
        )
        VALUES ($1, $2, $3, NOW())
        RETURNING id, user_id, lat, lon
    "#;

    let created_env = query_as::<_, Created>(sql)
        .bind(new_env.user_id)
        .bind(new_env.lat)
        .bind(new_env.lon)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("{:#?}", e);
            error::AppError::DatabaseError(e.to_string())
        })?;

    Ok(created_env)
}

pub async fn find_by_user_id(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<Entry>, error::AppError> {
    let sql = r#"
        SELECT id, user_id, lat, lon from point_conditions
        WHERE user_id = $1
    "#;
    let conditions = query_as::<_, Entry>(sql)
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("{:#?}", e);
            error::AppError::DatabaseError(e.to_string())
        })?;

    Ok(conditions)
}
