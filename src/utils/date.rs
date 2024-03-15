use chrono::NaiveDateTime;
use diesel::data_types::PgTimestamp;

use crate::error::{CustomError, ErrorCodes};

use super::HTTPStatusCode;

pub fn get_current_date_time() -> NaiveDateTime {
    chrono::Utc::now().naive_utc()
}

pub fn calculate_date_for_pg(date: chrono::NaiveDateTime) -> Result<PgTimestamp, CustomError> {
    let date_base = match calculate_difference_for_base() {
        Ok(date) => date,
        Err(e) => return Err(e),
    };

    Ok(PgTimestamp(date.and_utc().timestamp_micros() - date_base))
}

fn calculate_difference_for_base() -> Result<i64, CustomError> {
    let start_date_pg = match chrono::NaiveDate::from_ymd_opt(2000, 1, 1) {
        Some(date) => date,
        None => {
            return Err(CustomError::new(
                String::from("Unable to calculate start date"),
                ErrorCodes::DateCalculationError,
                HTTPStatusCode::InternalServerError,
            ))
        }
    };
    let start_time_pg = match chrono::NaiveTime::from_hms_opt(0, 0, 0) {
        Some(date) => date,
        None => {
            return Err(CustomError::new(
                String::from("Unable to calculate start time"),
                ErrorCodes::DateCalculationError,
                HTTPStatusCode::InternalServerError,
            ))
        }
    };
    Ok(chrono::NaiveDateTime::new(start_date_pg, start_time_pg)
        .and_utc()
        .timestamp_micros())
}
