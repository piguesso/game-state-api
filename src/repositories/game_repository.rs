use diesel::{
    delete, insert_into,
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

use crate::{
    database::{games, Game, NewGame, UpdateGame},
    error::CustomError,
    repositories::IRepository,
    types::RequestOptions,
    utils::calculate_offset,
};

pub struct GameRepository {
    database_connection: PooledConnection<ConnectionManager<PgConnection>>,
}

pub fn new_game_repository(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> GameRepository {
    GameRepository {
        database_connection: connection,
    }
}

impl IRepository<NewGame, UpdateGame, Game, i32, i32> for GameRepository {
    fn create_entry(&mut self, entry: NewGame) -> Result<i32, CustomError> {
        match insert_into(games::table)
            .values(entry)
            .returning(games::id)
            .get_result::<i32>(&mut self.database_connection)
        {
            Ok(id) => {
                info!("Created new game with id: {}", id);
                Ok(id)
            }
            Err(err) => {
                error!("Failed to create new game. \n With error: {}", err);
                Err(CustomError::InternalServerError(None))
            }
        }
    }

    fn delete_entry(&mut self, entry_id: i32) -> Option<CustomError> {
        match delete(games::table)
            .filter(games::id.eq(entry_id))
            .execute(&mut self.database_connection)
        {
            Ok(_) => {
                info!("Deleted game with id: {}", entry_id);
                None
            }
            Err(err) => {
                error!(
                    "Failed to delete game with id: {}. \n With error: {}",
                    entry_id, err
                );
                Some(CustomError::InternalServerError(None))
            }
        }
    }

    fn update_entry(&mut self, entry_id: i32, entry: UpdateGame) -> Result<Game, CustomError> {
        match update(games::table)
            .filter(games::id.eq(entry_id))
            .set(entry)
            .returning(games::all_columns)
            .get_result(&mut self.database_connection)
        {
            Ok(game) => {
                info!("Updated game with id: {}", entry_id);
                Ok(game)
            }
            Err(err) => {
                error!(
                    "Failed to update game with id: {}. \n With error: {}",
                    entry_id, err
                );
                Err(CustomError::InternalServerError(None))
            }
        }
    }

    fn fetch_entry(&mut self, entry_id: i32) -> Result<Game, CustomError> {
        match games::table
            .filter(games::id.eq(entry_id))
            .first(&mut self.database_connection)
        {
            Ok(game) => {
                info!("Fetched game with id: {}", entry_id);
                Ok(game)
            }
            Err(err) => {
                error!(
                    "Failed to fetch game with id: {}. \n With error: {}",
                    entry_id, err
                );
                Err(CustomError::InternalServerError(None))
            }
        }
    }

    fn fetch_entries(
        &mut self,
        request_options: RequestOptions<i32>,
    ) -> Result<Vec<Game>, CustomError> {
        let limit = match request_options.limit {
            Some(limit) => limit,
            None => 10,
        };

        let offset = match request_options.page {
            Some(page) => calculate_offset(limit, page),
            None => calculate_offset(limit, 1),
        };

        match games::table
            .limit(limit)
            .offset(offset)
            .order_by(games::id.desc())
            .load(&mut self.database_connection)
        {
            Ok(games) => {
                info!("Fetched games");
                Ok(games)
            }
            Err(err) => {
                error!("Failed to fetch games. \n With error: {}", err);
                Err(CustomError::InternalServerError(None))
            }
        }
    }
}
