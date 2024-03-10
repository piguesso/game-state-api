use crate::error::CustomError;

pub trait IGameStatus {
    fn to_string(&self) -> String;
    fn from_string(status: String) -> Result<GameStatus, CustomError>;
}

pub enum GameStatus {
    Waiting,
    Playing,
    Finished,
}

impl IGameStatus for GameStatus {
    fn to_string(&self) -> String {
        match self {
            GameStatus::Waiting => String::from("Waiting"),
            GameStatus::Playing => String::from("Playing"),
            GameStatus::Finished => String::from("Finished"),
        }
    }

    fn from_string(status: String) -> Result<GameStatus, CustomError> {
        match status.as_str() {
            "Waiting" => Ok(GameStatus::Waiting),
            "Playing" => Ok(GameStatus::Playing),
            "Finished" => Ok(GameStatus::Finished),
            _ => Err(CustomError::ErrorWithStatus(
                String::from("Invalid game status"),
                400,
            )),
        }
    }
}
