pub mod forecast;
pub mod marine_weather;

use tracing::error;

use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::middleware::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Geocode {
    pub latitude: f64,
    pub longitude: f64,
}

pub fn get_array_from_request_json<'a>(
    json: &'a Value,
    key: &str
) -> Result<&'a Vec<Value>, AppError> {
    json[key]
        .as_array()
        .ok_or_else(|| {
            let msg = format!("Invalid response format for key: {}", key);
            error!("{}", msg);
            AppError::InternalServerError(msg)
        }) 
}
