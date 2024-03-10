use crate::{
    database::{Game, NewGame, NewRound, Round, UpdateGame, UpdateRound}, error::CustomError, repositories::{IRepository, IStateHandler}
};

pub trait IGameStateService {
    fn start_game(&self, requester_id: String, game_id: u32) -> Option<CustomError>;
    fn pause_game(&self, requester_id: String, game_id: u32) -> Option<CustomError>;
    fn end_round(&self, requester_id: String, game_id: u32) -> Option<CustomError>;
    fn save_result(&self, requester_id: String, game_id: u32) -> Option<CustomError>;
}

pub struct GameStateService {
    state_handler: Box<dyn IStateHandler>,
    game_repository: Box<dyn IRepository<NewGame, UpdateGame, Game, i32, i64>>,
    round_repository: Box<dyn IRepository<NewRound, UpdateRound, Round, i32, i64>>,
}

impl IGameStateService for GameStateService {
    fn start_game(&self, requester_id: String, game_id: u32) -> Option<CustomError> {
        // TODO function
        unimplemented!()
    }

    fn pause_game(&self, requester_id: String, game_id: u32) -> Option<CustomError> {
        // TODO function
        unimplemented!()
    }

    fn end_round(&self, requester_id: String, game_id: u32) -> Option<CustomError> {
        // TODO function
        unimplemented!()
    }

    fn save_result(&self, requester_id: String, game_id: u32) -> Option<CustomError> {
        // TODO function
        unimplemented!()
    }
}
