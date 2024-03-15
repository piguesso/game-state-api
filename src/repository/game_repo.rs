use std::borrow::BorrowMut;

use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

use crate::{
    database::{games, Game, UpdateGame},
    error::{CustomError, ErrorCodes},
    utils::HTTPStatusCode,
};

pub trait IGameRepository<PK>: Send {
    fn update(&mut self, primary_key: PK, data: UpdateGame) -> Result<(), CustomError>;
    fn fetch_entry(&mut self, primary_key: PK) -> Result<Game, CustomError>;
}

pub fn new_game_repository(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> impl IGameRepository<i32> {
    GameRepository { connection }
}

pub struct GameRepository {
    connection: PooledConnection<ConnectionManager<PgConnection>>,
}

impl IGameRepository<i32> for GameRepository {
    fn update(&mut self, primary_key: i32, data: UpdateGame) -> Result<(), CustomError> {
        match update(games::table)
            .filter(games::id.eq(primary_key))
            .set(data)
            .execute(self.connection.borrow_mut())
        {
            Ok(_) => Ok(()),
            Err(_) => Err(CustomError::new(
                String::from("Unable to update game entry"),
                ErrorCodes::DatabaseUpdateError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entry(&mut self, primary_key: i32) -> Result<Game, CustomError> {
        match games::table
            .find(primary_key)
            .first(self.connection.borrow_mut())
        {
            Ok(game) => Ok(game),
            Err(_) => Err(CustomError::new(
                String::from("Unable to fetch game entry"),
                ErrorCodes::DatabaseFetchError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }
}
