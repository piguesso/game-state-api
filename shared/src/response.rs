use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T>
where
    T: Serialize,
{
    pub is_ok: bool,
    pub data: Option<T>,
    pub relations: Vec<Relation>,
}

#[derive(Serialize, Deserialize)]
pub struct Relation {
    pub route: String,
    pub method: String,
    pub resource: String,
}

#[derive(Serialize, Deserialize)]
pub struct GamePlayerStats {
    pub game_id: i32,
    pub player_id: String,
    pub round_stats: Vec<PlayerRoundStats>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerRoundStats {
    pub round_id: i32,
    pub score: i32,
    pub place: i32,
    pub is_winner: bool,
    pub time_used_to_complete: i64,
    pub first_topic: String,
    pub second_topic: String,
    pub third_topic: String,
    pub has_stopped_game: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RealtimeResponse<T> {
    pub event: String,
    pub data: Option<T>,
    pub error: Option<String>,
    pub error_code: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerJoined {
    pub player_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerLeft {
    pub player_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct GameStarted {
    pub game_status: String,
    pub round_number: i32,
    pub topic: String,
    pub start_time: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RoundFinished {
    pub round_number: i32,
    pub topic: String,
    pub start_time: i64,
    pub end_time: i64,
}

#[derive(Serialize, Deserialize)]
pub struct NextRoundStarted {
    pub round_number: i32,
    pub topic: String,
    pub start_time: i64,
}

#[derive(Serialize, Deserialize)]
pub struct GameFinished {
    pub game_status: String,
    pub player_stats: Vec<GamePlayerStats>,
}

#[derive(Serialize, Deserialize)]
pub struct RoundResultSent {
    pub round_number: i32,
    pub topic: String,
    pub start_time: i64,
    pub end_time: i64,
    pub player_stats: Vec<PlayerRoundStats>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorRes {}

pub enum ResponseEvents {
    PlayerJoined,
    PlayerLeft,
    GameStarted,
    RoundFinished,
    NextRoundStarted,
    GameFinished,
    RoundResultSent,
    Error,
}

impl ResponseEvents {
    pub fn to_string(&self) -> String {
        match self {
            ResponseEvents::PlayerJoined => "player_joined".to_string(),
            ResponseEvents::PlayerLeft => "player_left".to_string(),
            ResponseEvents::GameStarted => "game_started".to_string(),
            ResponseEvents::RoundFinished => "round_finished".to_string(),
            ResponseEvents::NextRoundStarted => "next_round_started".to_string(),
            ResponseEvents::GameFinished => "game_finished".to_string(),
            ResponseEvents::RoundResultSent => "round_result_sent".to_string(),
            ResponseEvents::Error => "error".to_string(),
        }
    }

    pub fn from_string(event: String) -> ResponseEvents {
        match event.as_str() {
            "player_joined" => ResponseEvents::PlayerJoined,
            "player_left" => ResponseEvents::PlayerLeft,
            "game_started" => ResponseEvents::GameStarted,
            "round_finished" => ResponseEvents::RoundFinished,
            "next_round_started" => ResponseEvents::NextRoundStarted,
            "game_finished" => ResponseEvents::GameFinished,
            "round_result_sent" => ResponseEvents::RoundResultSent,
            "error" => ResponseEvents::Error,
            _ => ResponseEvents::PlayerJoined,
        }
    }
}
