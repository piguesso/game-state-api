use std::borrow::{Borrow, BorrowMut};

use diesel::{
    insert_into,
    query_dsl::methods::FilterDsl,
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, PgConnection, RunQueryDsl,
};

use crate::{
    database::{players, NewPlayer, Player, UpdatePlayer},
    error::{CustomError, ErrorCodes},
    utils::HTTPStatusCode,
};

pub trait IPlayerRepository: Send {
    fn create(&mut self, data: NewPlayer) -> Result<(String, i32), CustomError>;
    fn update(&mut self, primary_key: (String, i32), data: UpdatePlayer)
        -> Result<(), CustomError>;
    fn fetch_entry(&mut self, primary_key: (String, i32)) -> Result<Player, CustomError>;
    fn fetch_entries(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError>;
    fn get_host(&mut self, game_id: i32) -> Result<Option<Player>, CustomError>;
}

pub fn new_player_repository(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> impl IPlayerRepository {
    PlayerRepository { connection }
}

pub struct PlayerRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl IPlayerRepository for PlayerRepository {
    fn create(&mut self, data: NewPlayer) -> Result<(String, i32), CustomError> {
        match insert_into(players::table)
            .values(data.borrow())
            .execute(self.connection.borrow_mut())
        {
            Ok(_) => Ok((data.player_id, data.game_id)),
            Err(_) => Err(CustomError::new(
                String::from("Unable to create player entry"),
                ErrorCodes::DatabaseInsertError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn update(
        &mut self,
        primary_key: (String, i32),
        data: UpdatePlayer,
    ) -> Result<(), CustomError> {
        match update(players::table)
            .filter(players::player_id.eq(primary_key.0))
            .filter(players::game_id.eq(primary_key.1))
            .set(data)
            .execute(self.connection.borrow_mut())
        {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::new(
                String::from("Unable to update player entry"),
                ErrorCodes::DatabaseUpdateError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entry(&mut self, primary_key: (String, i32)) -> Result<Player, CustomError> {
        match players::table
            .filter(players::player_id.eq(primary_key.0))
            .filter(players::game_id.eq(primary_key.1))
            .first(self.connection.borrow_mut())
        {
            Ok(player) => Ok(player),
            Err(err) => {
                if err == diesel::NotFound {
                    Err(CustomError::new(
                        String::from("Player entry not found"),
                        ErrorCodes::DatabaseFetchError,
                        HTTPStatusCode::NotFound,
                    ))
                } else {
                    Err(CustomError::new(
                        String::from("Unable to fetch player entry"),
                        ErrorCodes::DatabaseFetchError,
                        HTTPStatusCode::InternalServerError,
                    ))
                }
            }
        }
    }

    fn fetch_entries(&mut self, game_id: i32) -> Result<Vec<Player>, CustomError> {
        match players::table
            .filter(players::game_id.eq(game_id))
            .load(self.connection.borrow_mut())
        {
            Ok(players) => Ok(players),
            Err(_) => Err(CustomError::new(
                String::from("Unable to fetch player entries"),
                ErrorCodes::DatabaseFetchError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn get_host(&mut self, game_id: i32) -> Result<Option<Player>, CustomError> {
        match players::table
            .filter(players::game_id.eq(game_id))
            .filter(players::is_host.eq(true))
            .first(self.connection.borrow_mut())
        {
            Ok(player) => Ok(Some(player)),
            Err(err) => {
                if err == diesel::NotFound {
                    Ok(None)
                } else {
                    Err(CustomError::new(
                        String::from("Unable to fetch host"),
                        ErrorCodes::DatabaseFetchError,
                        HTTPStatusCode::InternalServerError,
                    ))
                }
            }
        }
    }
}
