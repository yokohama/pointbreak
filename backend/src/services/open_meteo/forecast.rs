use reqwest::Client;
use serde::{Serialize, Deserialize};

use crate::middleware::error;
use crate::services::open_meteo;

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub rain: f32,
    pub temperature: f32,
    pub weather_code: i32,
    pub wind_speed: f32,
}
impl Current {
    fn new(json: &serde_json::Value) -> Self {
        Self {
            rain: json["current"]["rain"]
                .as_f64().unwrap() as f32,
            temperature: json["current"]["temperature_2m"]
                .as_f64().unwrap() as f32,
            weather_code: json["current"]["weather_code"]
                .as_i64().unwrap() as i32,
            wind_speed: json["current"]["wind_speed_10m"]
                .as_f64().unwrap() as f32,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUnits {
    pub rain: String,
    pub temperature: String,
    pub weather_code: String,
    pub wind_speed: String,
}
impl CurrentUnits {
    fn new(json: &serde_json::Value) -> Self {
        Self {
            rain: json["current_units"]["rain"].to_string(),
            temperature: json["current_units"]["temperature"].to_string(),
            weather_code: json["current_units"]["wmo_code"].to_string(),
            wind_speed: json["wind_speed"].to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
    pub current: Current,
    pub units: CurrentUnits,
}

pub async fn fetch(
    geocode: &open_meteo::Geocode,
    timezone: &str
) -> Result<Forecast, error::AppError> {
    let client = Client::new();

    let url = format!(
        "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,precipitation,rain,weather_code,wind_speed_10m&timezone={}&forecast_days=1",
        geocode.latitude,
        geocode.longitude,
        timezone,
    );
    let res = client
        .get(url)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let current = Current::new(&res);
    let current_units = CurrentUnits::new(&res);

    Ok(Forecast {
        current,
        units: current_units,
    })
}
