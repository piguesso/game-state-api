pub enum GameStatus {
    WAITING,
    PLAYING,
    FINISHED,
}

impl GameStatus {
    pub fn from_string(status: String) -> GameStatus {
        match status.as_str() {
            "waiting" => GameStatus::WAITING,
            "playing" => GameStatus::PLAYING,
            "finished" => GameStatus::FINISHED,
            _ => GameStatus::WAITING,
        }
    }

    pub fn to_string(status: GameStatus) -> String {
        match status {
            GameStatus::WAITING => "waiting".to_string(),
            GameStatus::PLAYING => "playing".to_string(),
            GameStatus::FINISHED => "finished".to_string(),
        }
    }
}
