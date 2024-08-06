use chrono::{DateTime, Utc, FixedOffset, Timelike};
use crate::middleware::error::AppError;

pub fn get_current_time_jst() -> Result<DateTime<FixedOffset>, AppError> {
    let current_time_utc: DateTime<Utc> = Utc::now();
    let offset = FixedOffset::east_opt(9 * 3600)
        .ok_or_else(|| AppError::InternalServerError("Invalid offset".to_string()))?;

    Ok(current_time_utc.with_timezone(&offset))
}

pub fn get_current_time_utc_from_jst() -> Result<DateTime<Utc>, AppError> {
    let current_time_jst = get_current_time_jst()?;
    Ok(current_time_jst.with_timezone(&Utc))
}

pub fn truncate_to_hour(datetime: DateTime<Utc>) -> DateTime<Utc> {
    datetime.with_minute(0).unwrap().with_second(0).unwrap().with_nanosecond(0).unwrap()
}
