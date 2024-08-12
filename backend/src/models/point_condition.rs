use serde::Serialize;
use sqlx::{
    PgPool,
    query_as,
    FromRow,
};

use tracing::error;

use crate::middleware::error;

#[derive(Serialize)]
pub struct New {
    pub user_id: i32,
    pub lat: f64,
    pub lon: f64,
    pub time: String,
    pub swell_wave_height: f32,
    pub swell_wave_height_unit: String,
    pub swell_wave_direction: i32,
    pub swell_wave_direction_unit: String,
    pub rain: f32,
    pub rain_unit: String,
    pub temperature: f32,
    pub temperature_unit: String,
    pub weather_code: i32,
    pub weather_code_unit: String,
    pub wind_speed: f32,
    pub wind_speed_unit: String,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Created {
    id: i32,
    user_id: i32,
    lat: f64,
    lon: f64,
    time: String,
    swell_wave_height: f32,
    swell_wave_height_unit: String,
    swell_wave_direction: i32,
    swell_wave_direction_unit: String,
    rain: f32,
    rain_unit: String,
    temperature: f32,
    temperature_unit: String,
    weather_code: i32,
    weather_code_unit: String,
    wind_speed: f32,
    wind_speed_unit: String,
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
            swell_wave_height_unit,
            swell_wave_direction,
            swell_wave_direction_unit,
            rain,
            rain_unit,
            temperature,
            temperature_unit,
            weather_code,
            weather_code_unit,
            wind_speed,
            wind_speed_unit,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, NOW())
        RETURNING id, user_id, lat, lon, time, swell_wave_height, swell_wave_height_unit, swell_wave_direction, swell_wave_direction_unit, rain, rain_unit, temperature, temperature_unit, weather_code, weather_code_unit, wind_speed, wind_speed_unit
    "#;

    let created = query_as::<_, Created>(sql)
        .bind(new_condition.user_id)
        .bind(new_condition.lat)
        .bind(new_condition.lon)
        .bind(new_condition.time)
        .bind(new_condition.swell_wave_height)
        .bind(new_condition.swell_wave_height_unit)
        .bind(new_condition.swell_wave_direction)
        .bind(new_condition.swell_wave_direction_unit)
        .bind(new_condition.rain)
        .bind(new_condition.rain_unit)
        .bind(new_condition.temperature)
        .bind(new_condition.temperature_unit)
        .bind(new_condition.weather_code)
        .bind(new_condition.weather_code_unit)
        .bind(new_condition.wind_speed)
        .bind(new_condition.wind_speed_unit)
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

pub async fn all(
    pool: &PgPool
) -> Result<Vec<Entry>, error::AppError> {
    let sql = r#"
        SELECT * FROM point_conditions
    "#;
    let conditions = query_as::<_, Entry>(sql)
        .fetch_all(pool)
        .await
        .map_err(|e| {
            error!("{:#?}", e);
            error::AppError::DatabaseError(e.to_string())
        })?;

    Ok(conditions)
}
