use rocket::http::Status;
use serde::{Deserialize, Serialize};

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
