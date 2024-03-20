use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response<T>
where T: Serialize {
    pub is_ok: bool,
    pub data: Option<T>,
    pub relations: Vec<Relation>
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
    pub round_stats: Vec<PlayerRoundStats>
}

#[derive(Serialize, Deserialize)]
pub struct PlayerRoundStats {
    pub round_id: i32,
    pub score: i32,
    pub place: i32,
    pub is_winner: bool,
    pub time_used_to_complete: i32,
    pub first_topic: String,
    pub second_topic: String,
    pub third_topic: String,
    pub has_stopped_game: bool,
}
