use crate::error::Error;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Request {
    pub event: String,
    pub data: Option<RequestData>,
}

#[derive(Serialize, Deserialize)]
pub struct RequestData {
    pub round_number: Option<i32>,
    pub first_topic: Option<String>,
    pub second_topic: Option<String>,
    pub third_topic: Option<String>,
}

pub enum RequestEvent {
    JoinGame,
    LeaveGame,
    StartGame,
    FinishRound,
    StartNextRound,
    FinishGame,
    SendRoundResult,
}

impl RequestEvent {
    pub fn from_string(event: String) -> Result<RequestEvent, Error> {
        match event.as_str() {
            "join_game" => Ok(RequestEvent::JoinGame),
            "leave_game" => Ok(RequestEvent::LeaveGame),
            "start_game" => Ok(RequestEvent::StartGame),
            "finish_round" => Ok(RequestEvent::FinishRound),
            "start_next_round" => Ok(RequestEvent::StartNextRound),
            "finish_game" => Ok(RequestEvent::FinishGame),
            "send_round_result" => Ok(RequestEvent::SendRoundResult),
            _ => Err(Error::new(
                String::from("Invalid event type"),
                Status::BadRequest,
            )),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            RequestEvent::JoinGame => "join_game".to_string(),
            RequestEvent::LeaveGame => "leave_game".to_string(),
            RequestEvent::StartGame => "start_game".to_string(),
            RequestEvent::FinishRound => "finish_round".to_string(),
            RequestEvent::StartNextRound => "start_next_round".to_string(),
            RequestEvent::FinishGame => "finish_game".to_string(),
            RequestEvent::SendRoundResult => "send_round_result".to_string(),
        }
    }
}
