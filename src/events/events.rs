use serde::{Deserialize, Serialize};

pub struct InvalidEventTypeError {
    message: String,
}

impl InvalidEventTypeError {
    pub fn new() -> InvalidEventTypeError {
        InvalidEventTypeError {
            message: String::from("Invalid event type"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub enum EventType {
    GameStarted,
    RoundEnded,
    GamePaused,
    RoundStarted,
    GameEnded,
}

impl EventType {
    pub fn parse(event_type: i8) -> Result<EventType, InvalidEventTypeError> {
        match event_type {
            0 => Ok(EventType::GameStarted),
            1 => Ok(EventType::RoundEnded),
            2 => Ok(EventType::GamePaused),
            3 => Ok(EventType::RoundStarted),
            4 => Ok(EventType::GameEnded),
            _ => Err(InvalidEventTypeError::new()),
        }
    }

    pub fn to_integer(&self) -> i8 {
        match self {
            EventType::GameStarted => 0,
            EventType::RoundEnded => 1,
            EventType::GamePaused => 2,
            EventType::RoundStarted => 3,
            EventType::GameEnded => 4,
        }
    }
}
