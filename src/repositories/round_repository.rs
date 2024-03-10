use diesel::{
    delete, insert_into,
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

use crate::{
    database::{rounds, NewRound, Round, UpdateRound},
    utils::calculate_offset,
};

use super::IRepository;

pub struct RoundRepository {
    database_connection: PooledConnection<ConnectionManager<PgConnection>>,
}

pub fn new_round_repository(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> RoundRepository {
    RoundRepository {
        database_connection: connection,
    }
}

impl IRepository<NewRound, UpdateRound, Round, i32, i32> for RoundRepository {
    fn create_entry(&mut self, entry: NewRound) -> Result<i32, crate::error::CustomError> {
        match insert_into(rounds::table)
            .values(entry)
            .returning(rounds::id)
            .get_result::<i32>(&mut self.database_connection)
        {
            Ok(id) => {
                info!("Created new round with id: {}", id);
                Ok(id)
            }
            Err(err) => {
                error!("Failed to create new round. \n With error: {}", err);
                Err(crate::error::CustomError::InternalServerError(None))
            }
        }
    }

    fn delete_entry(&mut self, entry_id: i32) -> Option<crate::error::CustomError> {
        match delete(rounds::table)
            .filter(rounds::id.eq(entry_id))
            .execute(&mut self.database_connection)
        {
            Ok(_) => {
                info!("Deleted round with id: {}", entry_id);
                None
            }
            Err(err) => {
                error!(
                    "Failed to delete round with id: {}. \n With error: {}",
                    entry_id, err
                );
                Some(crate::error::CustomError::InternalServerError(None))
            }
        }
    }

    fn update_entry(
        &mut self,
        entry_id: i32,
        entry: UpdateRound,
    ) -> Result<Round, crate::error::CustomError> {
        match update(rounds::table)
            .filter(rounds::id.eq(entry_id))
            .set(entry)
            .get_result(&mut self.database_connection)
        {
            Ok(round) => {
                info!("Updated round with id: {}", entry_id);
                Ok(round)
            }
            Err(err) => {
                error!(
                    "Failed to update round with id: {}. \n With error: {}",
                    entry_id, err
                );
                Err(crate::error::CustomError::InternalServerError(None))
            }
        }
    }

    fn fetch_entry(&mut self, entry_id: i32) -> Result<Round, crate::error::CustomError> {
        match rounds::table
            .filter(rounds::id.eq(entry_id))
            .first(&mut self.database_connection)
        {
            Ok(round) => {
                info!("Fetched round with id: {}", entry_id);
                Ok(round)
            }
            Err(err) => {
                error!(
                    "Failed to fetch round with id: {}. \n With error: {}",
                    entry_id, err
                );
                Err(crate::error::CustomError::InternalServerError(None))
            }
        }
    }

    fn fetch_entries(
        &mut self,
        request_options: crate::types::RequestOptions<i32>,
    ) -> Result<Vec<Round>, crate::error::CustomError> {
        let limit = match request_options.limit {
            Some(limit) => limit,
            None => 10,
        };

        let offset = match request_options.page {
            Some(page) => calculate_offset(limit, page),
            None => calculate_offset(limit, 0),
        };

        match request_options.filter_value {
            Some(game_id) => match rounds::table
                .filter(rounds::game_id.eq(game_id))
                .limit(limit)
                .offset(offset)
                .load::<Round>(&mut self.database_connection)
            {
                Ok(rounds) => {
                    info!("Fetched {} rounds", rounds.len());
                    Ok(rounds)
                }
                Err(err) => {
                    error!("Failed to fetch rounds. \n With error: {}", err);
                    Err(crate::error::CustomError::InternalServerError(None))
                }
            },
            None => match rounds::table
                .limit(limit)
                .offset(offset)
                .load::<Round>(&mut self.database_connection)
            {
                Ok(rounds) => {
                    info!("Fetched {} rounds", rounds.len());
                    Ok(rounds)
                }
                Err(err) => {
                    error!("Failed to fetch rounds. \n With error: {}", err);
                    Err(crate::error::CustomError::InternalServerError(None))
                }
            },
        }
    }
}
