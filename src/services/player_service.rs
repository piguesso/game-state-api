use diesel::data_types::PgTimestamp;

use crate::{
    database::{NewPlayer, Player, UpdatePlayer},
    error::CustomError,
    repositories::{IRepository, IStateHandler, PlayerPrimaryKey},
    types::RequestOptions,
};

pub trait IPlayerService {
    fn add_player_to_game(
        &mut self,
        game_id: i32,
        player_id: String,
        is_host: bool,
    ) -> Option<CustomError>;
    fn remove_player_from_game(&mut self, game_id: i32, player_id: String) -> Option<CustomError>;
    fn get_players_in_game(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError>;
    fn get_current_players_in_game(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError>;
}

pub struct PlayerService {
    state_handler: Box<dyn IStateHandler>,
    player_repository: Box<dyn IRepository<NewPlayer, UpdatePlayer, Player, PlayerPrimaryKey, i32>>,
}

impl IPlayerService for PlayerService {
    fn add_player_to_game(
        &mut self,
        game_id: i32,
        player_id: String,
        is_host: bool,
    ) -> Option<CustomError> {
        // TODO validate player_id using user database or clerk service

        match self
            .player_repository
            .fetch_entry((player_id.clone(), game_id))
        {
            Ok(_) => {
                return Some(CustomError::BadRequest(String::from(
                    "Player already in game",
                )))
            }
            Err(_) => (),
        };

        let new_player = NewPlayer {
            player_id: player_id.clone(),
            game_id,
            is_host: Some(is_host),
        };

        match self.player_repository.create_entry(new_player) {
            Ok(_) => (),
            Err(e) => return Some(e),
        };

        match self
            .state_handler
            .add_to_set(String::from(format!("game:{}:players", game_id)), player_id)
        {
            Some(err) => Some(err),
            None => None,
        }
    }

    fn remove_player_from_game(&mut self, game_id: i32, player_id: String) -> Option<CustomError> {
        let updated_player = UpdatePlayer {
            player_id: player_id.clone(),
            game_id,
            is_host: None,
            left_game_at: Some(PgTimestamp(chrono::Utc::now().naive_utc().timestamp())),
        };

        match self
            .player_repository
            .update_entry((player_id.clone(), game_id), updated_player)
        {
            Ok(_) => (),
            Err(e) => return Some(e),
        };

        match self
            .state_handler
            .remove_from_set(String::from(format!("game:{}:players", game_id)), player_id)
        {
            Some(err) => Some(err),
            None => None,
        }
    }

    fn get_players_in_game(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError> {
        let request_options: RequestOptions<i32> = RequestOptions {
            limit: None,
            page: None,
            order: None,
            order_field: None,
            search: None,
            filter_field: None,
            filter_value: Some(game_id),
        };

        match self.player_repository.fetch_entries(request_options) {
            Ok(players) => Ok(players),
            Err(e) => Err(e),
        }
    }

    fn get_current_players_in_game(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError> {
        let player_ids = match self
            .state_handler
            .get_members_of_set(String::from(format!("game:{}:players", game_id)))
        {
            Ok(ids) => ids,
            Err(err) => return Err(err),
        };

        let request_options: RequestOptions<i32> = RequestOptions {
            limit: None,
            page: None,
            order: None,
            order_field: None,
            search: None,
            filter_field: None,
            filter_value: Some(game_id),
        };

        let players = match self.player_repository.fetch_entries(request_options) {
            Ok(players) => players,
            Err(e) => return Err(e),
        };

        let filtered_list = players
            .into_iter()
            .filter(|player| player_ids.contains(&player.player_id))
            .collect();

        Ok(filtered_list)
    }
}
