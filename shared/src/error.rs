use serde::{Deserialize, Serialize};
use rocket::http::Status;

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub status_code: Status,
}

impl Error {
    pub fn new(message: String, status_code: Status) -> Error {
        Error {
            message,
            status_code,
        }
    }
}
