use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub enum CustomError {
    ErrorWithStatus(String, u16),
    InternalServerError(Option<String>),
    BadRequest(String),
}

impl Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            CustomError::ErrorWithStatus(ref msg, status) => write!(f, "{}: {}", status, msg),
            CustomError::InternalServerError(ref msg) => {
                let message = msg.clone();
                write!(
                    f,
                    "{}",
                    message.unwrap_or(String::from("Internal server error"))
                )
            }
            CustomError::BadRequest(ref msg) => write!(f, "400: {}", msg),
        }
    }
}

impl Error for CustomError {}
