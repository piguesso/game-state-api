use crate::utils::HTTPStatusCode;

pub enum ErrorCodes {
    InvalidMessageType = 1,
    InvalidEventType = 2,
    DatabaseInsertError = 3,
    DatabaseDeleteError = 4,
    DatabaseUpdateError = 5,
    DatabaseFetchError = 6,
    NotAllowedToPerformAction = 7,
    InvalidInput = 8,
    DateCalculationError = 9,
}

pub struct CustomError {
    pub message: String,
    pub code: ErrorCodes,
    pub status_code: HTTPStatusCode,
}

impl CustomError {
    pub fn new(message: String, code: ErrorCodes, status_code: HTTPStatusCode) -> CustomError {
        CustomError {
            message,
            code,
            status_code,
        }
    }
}
