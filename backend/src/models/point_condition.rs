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
    pub time: String,
    pub swell_wave_height: f32,
    pub swell_wave_direction: i32,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Created {
    id: i32,
    user_id: i32,
    lat: f64,
    lon: f64,
    time: String,
    swell_wave_height: f32,
    swell_wave_direction: i32,
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
    new_condition: New
) -> Result<Created, error::AppError> {
    let sql = r#"
        INSERT INTO point_conditions (
            user_id, 
            lat,
            lon,
            time,
            swell_wave_height,
            swell_wave_direction,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING id, user_id, lat, lon, time, swell_wave_height, swell_wave_direction
    "#;

    let created = query_as::<_, Created>(sql)
        .bind(new_condition.user_id)
        .bind(new_condition.lat)
        .bind(new_condition.lon)
        .bind(new_condition.time)
        .bind(new_condition.swell_wave_height)
        .bind(new_condition.swell_wave_direction)
        .fetch_one(pool)
        .await
        .map_err(|e| {
            error!("{:#?}", e);
            error::AppError::DatabaseError(e.to_string())
        })?;

    Ok(created)
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
