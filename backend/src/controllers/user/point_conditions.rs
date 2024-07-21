use axum::{ 
    extract::State,
    response::Json,
};
use serde::Serialize;
use sqlx::PgPool;

use crate::{
    midleware::{auth, error}, 
    models, 
    requests, 
    services::open_meteo::{
        self,
        marine_weather,
        forecast,
    },
};

pub async fn index(
    State(pool): State<PgPool>,
    claims: auth::Claims,
) -> Result<Json<impl Serialize>, error::AppError> {
    let current_user = claims.get_current_user(&pool).await?;
    let point_conditions: Vec<_> = models::point_condition::find_by_user_id(
        &pool, 
        current_user.id
    ).await?;
    Ok(Json(point_conditions))
}

pub async fn create(
    State(pool): State<PgPool>,
    claims: auth::Claims,
    Json(payload): Json<requests::point_condition::New>,
) -> Result<Json<impl Serialize>, error::AppError> {

    let geocode = open_meteo::Geocode {
        latitude: payload.lat,
        longitude: payload.lon,
    };

    let forecast = forecast::fetch(
        &geocode,
        &payload.timezone,
    ).await?;

    let marine_weather = marine_weather::fetch(
        &geocode,
        &payload.start_date,
        &payload.end_date,
        &payload.timezone,
    ).await?;

    let current_user = claims.get_current_user(&pool).await?;
    let new = models::point_condition::New { 
        user_id: current_user.id, 
        lat: geocode.latitude,
        lon: geocode.longitude,
        time: marine_weather.time,
        swell_wave_height: marine_weather.swell_wave_height,
        swell_wave_direction: marine_weather.swell_wave_direction,
    };
    let created = models::point_condition::create(&pool, new).await?;

    Ok(Json(created))
}
