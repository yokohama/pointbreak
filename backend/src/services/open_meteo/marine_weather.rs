use chrono::{DateTime, NaiveDateTime, Utc};

use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::midleware::error;
use crate::utils;
use crate::services::open_meteo::{
    Geocode,
    get_array_from_request_json,
};

#[derive(Debug, Deserialize)]
pub struct HourlyUnits {
    pub time: String,
    pub swell_wave_height: String,
    pub swell_wave_direction: String,
}

#[derive(Debug, Deserialize)]
pub struct Hourly {
    pub time: Vec<String>,
    pub swell_wave_height: Vec<f32>,
    pub swell_wave_direction: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub hourly_units: HourlyUnits,
    pub hourly: Hourly,
}

#[derive(Debug, Serialize)]
pub struct MarineWeather {
    pub time: String,
    pub swell_wave_height: f32,
    pub swell_wave_direction: i32,
}

pub async fn fetch(
    geocode: &Geocode,
    start_date: &str, 
    end_date: &str, 
    timezone: &str
) -> Result<MarineWeather, error::AppError> {
    let client = Client::new();

    let url = format!(
        "https://marine-api.open-meteo.com/v1/marine?latitude={}&longitude={}&hourly=swell_wave_height,swell_wave_direction&timezone={}&start_date={}&end_date={}",
        geocode.latitude,
        geocode.longitude,
        timezone,
        start_date,
        end_date,
    );

    let res = client
        .get(&url)
        .send().await?
        .json::<serde_json::Value>()
        .await?;

    let times = get_array_from_request_json(&res["hourly"], "time")?;
    let swell_wave_heights = get_array_from_request_json(
        &res["hourly"], 
        "swell_wave_height"
    )?;
    let swell_wave_directions = get_array_from_request_json(
        &res["hourly"], 
        "swell_wave_direction"
    )?;

    let index = find_time_element_current_time_index(&times)
        .ok_or(error::AppError::InternalServerError("No match time element found.".to_string()))?;

    Ok(MarineWeather {
        time: times[index].as_str().unwrap().to_string(),
        swell_wave_height: swell_wave_heights[index]
            .as_f64().unwrap() as f32,
        swell_wave_direction: swell_wave_directions[index]
            .as_i64().unwrap() as i32,
    })
}

fn find_time_element_current_time_index(
    time_elements: &[serde_json::Value], 
) -> Option<usize> {
    let current_time_utc = match utils::get_current_time_utc_from_jst() {
        Ok(time) => utils::truncate_to_hour(time),
        Err(_) => return None,
    };

    for (i, time_str) in time_elements.iter().enumerate() {
        match NaiveDateTime::parse_from_str(
            time_str.as_str().unwrap(), 
            "%Y-%m-%dT%H:%M"
        ) {
            Ok(naive_time) => {
                let time = DateTime::<Utc>::from_utc(naive_time, Utc);
                if time == current_time_utc {
                    return Some(i);
                }
            },
            Err(_) => {
                continue;
            }
        }
    }
    None
}
