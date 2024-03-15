pub struct PlayerScorePerGame {
    pub game_id: i32,
    pub player_id: String,
    pub score: i32,
    pub position: i32,
    pub score_per_round: Vec<PlayerScorePerRound>,
}

pub struct PlayerScorePerRound {
    pub round_id: i32,
    pub score: i32,
    pub position: i32,
    pub round_number: i32,
}

pub struct PlayerScoringValues {
    pub time_completed: chrono::NaiveDateTime,
    pub first_topic: String,
    pub second_topic: String,
    pub third_topic: String,
    pub has_stopped_game: bool,
}
