use crate::{
    database::{Game, NewGame, UpdateGame},
    error::CustomError,
    repositories::{IRepository, IStateHandler},
    types::{GameStatus, IGameStatus, RequestOptions},
};

pub trait IGameService {
    fn create_game(&mut self, requester_id: String, game: NewGame) -> Result<i32, CustomError>;
    fn get_game(&mut self, game_id: i64) -> Result<Game, CustomError>;
    fn get_games(&mut self, request_options: RequestOptions<i64>)
        -> Result<Vec<Game>, CustomError>;
}

pub struct GameService {
    state_handler: Box<dyn IStateHandler>,
    game_repository: Box<dyn IRepository<NewGame, UpdateGame, Game, i32, i64>>,
}

impl IGameService for GameService {
    fn create_game(&mut self, requester_id: String, game: NewGame) -> Result<i32, CustomError> {
        let status = match GameStatus::from_string(game.status.clone()) {
            Ok(status) => status,
            Err(e) => return Err(e),
        };

        // TODO Check if requester_id and game.winner_id is valid with Clerk service

        let game_id = match self.game_repository.create_entry(game) {
            Ok(game_id) => game_id,
            Err(e) => return Err(e),
        };

        match self
            .state_handler
            .add_to_set(String::from("games"), game_id.to_string())
        {
            None => (),
            Some(e) => return Err(e),
        };

        match self.state_handler.write_value(
            format!("game:{}:state", game_id.to_string()),
            status.to_string(),
        ) {
            None => (),
            Some(e) => return Err(e),
        };

        Ok(game_id)
    }

    fn get_game(&mut self, game_id: i64) -> Result<Game, CustomError> {
        let mut game = match self.game_repository.fetch_entry(game_id as i32) {
            Ok(game) => game,
            Err(e) => return Err(e),
        };

        match self
            .state_handler
            .read_value(String::from(format!("game:{}:state", game_id.to_string())))
        {
            Ok(status) => {
                let status = match GameStatus::from_string(status) {
                    Ok(status) => status,
                    Err(e) => return Err(e),
                };

                game = Game {
                    status: status.to_string(),
                    ..game
                };
            }
            Err(_) => (),
        };

        Ok(game)
    }

    fn get_games(
        &mut self,
        request_options: RequestOptions<i64>,
    ) -> Result<Vec<Game>, CustomError> {
        let games = match self.game_repository.fetch_entries(request_options) {
            Ok(games) => games,
            Err(e) => return Err(e),
        };

        let mut result = Vec::new();

        for game in games {
            let status = match self
                .state_handler
                .read_value(String::from(format!("game:{}:state", game.id.to_string())))
            {
                Ok(status) => status,
                Err(_) => String::from("Finished"),
            };

            let status = match GameStatus::from_string(status) {
                Ok(status) => status,
                Err(_) => GameStatus::Finished,
            };

            result.push(Game {
                status: status.to_string(),
                ..game
            });
        }

        Ok(result)
    }
}
