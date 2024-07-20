use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct New {
    pub lat: f64,
    pub lon: f64,
    pub start_date: String, 
    pub end_date: String,
    pub timezone: String,
}
