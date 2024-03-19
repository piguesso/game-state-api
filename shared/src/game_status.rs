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
}
