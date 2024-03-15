use crate::{
    database::{Game, UpdateGame},
    error::CustomError,
    repository::IGameRepository,
};

pub trait IGameService: Send {
    fn update(&mut self, primary_key: i32, data: UpdateGame) -> Result<(), CustomError>;
    fn fetch_entry(&mut self, primary_key: i32) -> Result<Game, CustomError>;
}

pub fn new_game_service(game_repo: Box<dyn IGameRepository<i32>>) -> Box<dyn IGameService> {
    Box::new(GameService { game_repo })
}

pub struct GameService {
    game_repo: Box<dyn IGameRepository<i32>>,
}

impl IGameService for GameService {
    fn update(&mut self, primary_key: i32, data: UpdateGame) -> Result<(), CustomError> {
        self.game_repo.update(primary_key, data)
    }

    fn fetch_entry(&mut self, primary_key: i32) -> Result<Game, CustomError> {
        self.game_repo.fetch_entry(primary_key)
    }
}
