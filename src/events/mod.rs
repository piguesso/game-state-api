use uuid::Uuid;

use crate::{
    error::{CustomError, ErrorCodes},
    utils::HTTPStatusCode,
};

pub enum EventType {
    GameStarted,
    GameEnded,
    GamePaused,
    RoundStarted,
    RoundEnded,
    PlayerJoined,
    PlayerLeft,
}

impl EventType {
    pub fn from_string(event_type: String) -> Result<EventType, CustomError> {
        match event_type.as_str() {
            "game_started" => Ok(EventType::GameStarted),
            "game_ended" => Ok(EventType::GameEnded),
            "game_paused" => Ok(EventType::GamePaused),
            "round_started" => Ok(EventType::RoundStarted),
            "round_ended" => Ok(EventType::RoundEnded),
            "player_joined" => Ok(EventType::PlayerJoined),
            "player_left" => Ok(EventType::PlayerLeft),
            _ => Err(CustomError::new(
                String::from("Invalid event type"),
                ErrorCodes::InvalidEventType,
                HTTPStatusCode::BadRequest,
            )),
        }
    }

    fn to_string(&self) -> String {
        match self {
            EventType::GameStarted => "game_started".to_string(),
            EventType::GameEnded => "game_ended".to_string(),
            EventType::GamePaused => "game_paused".to_string(),
            EventType::RoundStarted => "round_started".to_string(),
            EventType::RoundEnded => "round_ended".to_string(),
            EventType::PlayerJoined => "player_joined".to_string(),
            EventType::PlayerLeft => "player_left".to_string(),
        }
    }
}

pub struct Message {
    pub id: String,
    pub channel: String,
    pub event: EventType,
    pub requester_id: String,
}

fn generate_id() -> String {
    Uuid::new_v4().to_string()
}

impl Message {
    fn new(channel: String, event: EventType, requester_id: String) -> Message {
        Message {
            id: generate_id(),
            channel,
            event,
            requester_id,
        }
    }
}
