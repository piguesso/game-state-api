use std::borrow::{Borrow, BorrowMut};

use diesel::{
    delete, insert_into,
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

use crate::{
    database::{
        player_scoring_round, NewPlayerScoringRound, PlayerScoringRound, UpdatePlayerScoringRound,
    },
    error::CustomError,
};

pub trait IPlayerRoundScoringRepository: Send {
    fn create(&mut self, data: NewPlayerScoringRound) -> Result<(String, i32, i32), CustomError>;
    fn delete(&mut self, primary_key: (String, i32, i32)) -> Result<(), CustomError>;
    fn update(
        &mut self,
        primary_key: (String, i32, i32),
        data: UpdatePlayerScoringRound,
    ) -> Result<(), CustomError>;
    fn fetch_entry(
        &mut self,
        primary_key: (String, i32, i32),
    ) -> Result<PlayerScoringRound, CustomError>;
    fn fetch_entries(&mut self, game_id: i32) -> Result<Vec<PlayerScoringRound>, CustomError>;
    fn fetch_entries_with_player(
        &mut self,
        game_id: i32,
        player_id: String,
    ) -> Result<Vec<PlayerScoringRound>, CustomError>;
}

pub fn new_player_round_scoring_repository(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> impl IPlayerRoundScoringRepository {
    PlayerRoundScoringRepository { connection }
}

pub struct PlayerRoundScoringRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl IPlayerRoundScoringRepository for PlayerRoundScoringRepository {
    fn create(&mut self, data: NewPlayerScoringRound) -> Result<(String, i32, i32), CustomError> {
        match insert_into(player_scoring_round::table)
            .values(data.borrow())
            .execute(self.connection.borrow_mut())
        {
            Ok(_) => Ok((data.player_id, data.game_id, data.round_id)),
            Err(_) => Err(CustomError::new(
                String::from("Unable to create player scoring round entry"),
                crate::error::ErrorCodes::DatabaseInsertError,
                crate::utils::HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn delete(&mut self, primary_key: (String, i32, i32)) -> Result<(), CustomError> {
        match delete(player_scoring_round::table)
            .filter(player_scoring_round::player_id.eq(primary_key.0))
            .filter(player_scoring_round::game_id.eq(primary_key.1))
            .filter(player_scoring_round::round_id.eq(primary_key.2))
            .execute(self.connection.borrow_mut())
        {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::new(
                String::from("Unable to delete player scoring round entry"),
                crate::error::ErrorCodes::DatabaseDeleteError,
                crate::utils::HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn update(
        &mut self,
        primary_key: (String, i32, i32),
        data: UpdatePlayerScoringRound,
    ) -> Result<(), CustomError> {
        match update(player_scoring_round::table)
            .filter(player_scoring_round::player_id.eq(primary_key.0))
            .filter(player_scoring_round::game_id.eq(primary_key.1))
            .filter(player_scoring_round::round_id.eq(primary_key.2))
            .set(data)
            .execute(self.connection.borrow_mut())
        {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::new(
                String::from("Unable to update player scoring round entry"),
                crate::error::ErrorCodes::DatabaseUpdateError,
                crate::utils::HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entry(
        &mut self,
        primary_key: (String, i32, i32),
    ) -> Result<PlayerScoringRound, CustomError> {
        match player_scoring_round::table
            .filter(player_scoring_round::player_id.eq(primary_key.0))
            .filter(player_scoring_round::game_id.eq(primary_key.1))
            .filter(player_scoring_round::round_id.eq(primary_key.2))
            .first(self.connection.borrow_mut())
        {
            Ok(player_round_scoring) => Ok(player_round_scoring),
            Err(_) => Err(CustomError::new(
                String::from("Unable to fetch player scoring round entry"),
                crate::error::ErrorCodes::DatabaseFetchError,
                crate::utils::HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entries_with_player(
        &mut self,
        game_id: i32,
        player_id: String,
    ) -> Result<Vec<PlayerScoringRound>, CustomError> {
        match player_scoring_round::table
            .filter(player_scoring_round::game_id.eq(game_id))
            .filter(player_scoring_round::player_id.eq(player_id))
            .load::<PlayerScoringRound>(self.connection.borrow_mut())
        {
            Ok(player_round_scorings) => Ok(player_round_scorings),
            Err(_) => Err(CustomError::new(
                String::from("Unable to fetch player scoring round entries"),
                crate::error::ErrorCodes::DatabaseFetchError,
                crate::utils::HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entries(&mut self, game_id: i32) -> Result<Vec<PlayerScoringRound>, CustomError> {
        match player_scoring_round::table
            .filter(player_scoring_round::game_id.eq(game_id))
            .load::<PlayerScoringRound>(self.connection.borrow_mut())
        {
            Ok(player_round_scorings) => Ok(player_round_scorings),
            Err(_) => Err(CustomError::new(
                String::from("Unable to fetch player scoring round entries"),
                crate::error::ErrorCodes::DatabaseFetchError,
                crate::utils::HTTPStatusCode::InternalServerError,
            )),
        }
    }
}
