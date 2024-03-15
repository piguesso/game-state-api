use std::borrow::BorrowMut;

use diesel::{
    query_dsl::methods::FilterDsl,
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, PgConnection, RunQueryDsl,
};

use crate::{
    database::{player_scoring, PlayerScoring, UpdatePlayerScoring},
    error::{CustomError, ErrorCodes},
    utils::HTTPStatusCode,
};

pub trait IPlayerScoringRepository: Send {
    fn update(&mut self, primary_key: String, data: UpdatePlayerScoring)
        -> Result<(), CustomError>;
    fn fetch_entry(&mut self, primary_key: String) -> Result<PlayerScoring, CustomError>;
}

pub fn new_player_scoring_repository(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> impl IPlayerScoringRepository {
    PlayerScoringRepository { connection }
}

pub struct PlayerScoringRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl IPlayerScoringRepository for PlayerScoringRepository {
    fn update(
        &mut self,
        primary_key: String,
        data: UpdatePlayerScoring,
    ) -> Result<(), CustomError> {
        match update(player_scoring::table)
            .filter(player_scoring::player_id.eq(primary_key))
            .set(data)
            .execute(self.connection.borrow_mut())
        {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::new(
                String::from("Failed to update player scoring"),
                ErrorCodes::DatabaseUpdateError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entry(&mut self, primary_key: String) -> Result<PlayerScoring, CustomError> {
        match player_scoring::table
            .filter(player_scoring::player_id.eq(primary_key))
            .first(self.connection.borrow_mut())
        {
            Ok(entry) => Ok(entry),
            Err(_) => Err(CustomError::new(
                String::from("Failed to fetch player scoring"),
                ErrorCodes::DatabaseFetchError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }
}
