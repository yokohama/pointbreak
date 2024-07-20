use reqwest::Client;
use serde::Deserialize;

use crate::midleware::error;

#[derive(Debug, Deserialize)]
pub struct HourlyUnits {
    pub time: String,
    pub swell_wave_height: String,
    pub swell_wave_direction: String,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub hourly_units: HourlyUnits,
}

pub async fn get_marine_weather(
    lat: f64, 
    lon: f64, 
    start_date: &str, 
    end_date: &str, 
    timezone: &str
) -> Result<WeatherResponse, error::AppError> {
    let client = Client::new();

    let url = format!(
        "https://marine-api.open-meteo.com/v1/marine?latitude={}&longitude={}&hourly=swell_wave_height,swell_wave_direction&timezone={}&start_date={}&end_date={}",
        lat,
        lon,
        timezone,
        start_date,
        end_date,
    );

    let res = client
        .get(&url)
        .send()
        .await?
        .json::<WeatherResponse>()
        .await?;

    Ok(res)
}

