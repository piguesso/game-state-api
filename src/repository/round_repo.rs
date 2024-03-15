use std::borrow::BorrowMut;

use diesel::{
    insert_into,
    r2d2::{ConnectionManager, PooledConnection},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

use crate::{
    database::{rounds, NewRound, Round, UpdateRound},
    error::{CustomError, ErrorCodes},
    utils::HTTPStatusCode,
};

pub trait IRoundRepository: Send {
    fn create(&mut self, data: NewRound) -> Result<i32, CustomError>;
    fn update(&mut self, primary_key: i32, data: UpdateRound) -> Result<i32, CustomError>;
    fn fetch_entry(&mut self, primary_key: i32) -> Result<Round, CustomError>;
    fn fetch_entries(&mut self, game_id: i32) -> Result<Vec<Round>, CustomError>;
}

pub fn new_round_repository(
    conn: PooledConnection<ConnectionManager<PgConnection>>,
) -> RoundRepository {
    RoundRepository { conn }
}

pub struct RoundRepository {
    conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl IRoundRepository for RoundRepository {
    fn create(&mut self, data: NewRound) -> Result<i32, CustomError> {
        match insert_into(rounds::table)
            .values(data)
            .returning(rounds::id)
            .get_result(self.conn.borrow_mut())
        {
            Ok(id) => Ok(id),
            Err(_) => Err(CustomError::new(
                String::from("Error creating round"),
                ErrorCodes::DatabaseInsertError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn update(&mut self, primary_key: i32, data: UpdateRound) -> Result<i32, CustomError> {
        match diesel::update(rounds::table.find(primary_key))
            .set(data)
            .returning(rounds::id)
            .get_result(self.conn.borrow_mut())
        {
            Ok(id) => Ok(id),
            Err(_) => Err(CustomError::new(
                String::from("Error updating round"),
                ErrorCodes::DatabaseUpdateError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entry(&mut self, primary_key: i32) -> Result<Round, CustomError> {
        match rounds::table
            .find(primary_key)
            .first(self.conn.borrow_mut())
        {
            Ok(round) => Ok(round),
            Err(_) => Err(CustomError::new(
                String::from("Error fetching round"),
                ErrorCodes::DatabaseFetchError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }

    fn fetch_entries(&mut self, game_id: i32) -> Result<Vec<Round>, CustomError> {
        match rounds::table
            .filter(rounds::game_id.eq(game_id))
            .load(self.conn.borrow_mut())
        {
            Ok(rounds) => Ok(rounds),
            Err(_) => Err(CustomError::new(
                String::from("Error fetching rounds"),
                ErrorCodes::DatabaseFetchError,
                HTTPStatusCode::InternalServerError,
            )),
        }
    }
}
