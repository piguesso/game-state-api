use crate::{
    database::{players, NewPlayer, Player, UpdatePlayer},
    error::CustomError,
    types::RequestOptions,
    utils::calculate_offset,
};
use diesel::{
    delete, insert_into,
    r2d2::{ConnectionManager, PooledConnection},
    update, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

use super::IRepository;

pub type PlayerPrimaryKey = (String, i32);

pub struct PlayerRepository {
    database_connection: PooledConnection<ConnectionManager<PgConnection>>,
}

pub fn new_player_repository(
    connection: PooledConnection<ConnectionManager<PgConnection>>,
) -> PlayerRepository {
    PlayerRepository {
        database_connection: connection,
    }
}

impl IRepository<NewPlayer, UpdatePlayer, Player, PlayerPrimaryKey, i32> for PlayerRepository {
    fn create_entry(&mut self, entry: NewPlayer) -> Result<PlayerPrimaryKey, CustomError> {
        match insert_into(players::table)
            .values(entry)
            .returning((players::player_id, players::game_id))
            .get_result::<(String, i32)>(&mut self.database_connection)
        {
            Ok(primary_key) => {
                info!(
                    "Created new player with id: {} for game: {}",
                    primary_key.0, primary_key.1
                );
                Ok(primary_key)
            }
            Err(err) => {
                error!("Failed to create new player. \n With error: {}", err);
                Err(CustomError::InternalServerError(None))
            }
        }
    }

    fn delete_entry(&mut self, entry_id: PlayerPrimaryKey) -> Option<CustomError> {
        match delete(players::table)
            .filter(players::player_id.eq(entry_id.0.clone()))
            .filter(players::game_id.eq(entry_id.1))
            .execute(&mut self.database_connection)
        {
            Ok(_) => {
                info!(
                    "Deleted player with id: {} in game: {}",
                    entry_id.0, entry_id.1
                );
                None
            }
            Err(err) => {
                error!(
                    "Failed to delete player with id: {} in game: {}. \n With error: {}",
                    entry_id.0, entry_id.1, err
                );
                Some(CustomError::InternalServerError(None))
            }
        }
    }

    fn update_entry(
        &mut self,
        entry_id: PlayerPrimaryKey,
        entry: UpdatePlayer,
    ) -> Result<Player, CustomError> {
        match update(players::table)
            .filter(players::player_id.eq(entry_id.0.clone()))
            .filter(players::game_id.eq(entry_id.1))
            .set(entry)
            .get_result(&mut self.database_connection)
        {
            Ok(player) => {
                info!(
                    "Updated player with id: {} in game: {}",
                    entry_id.0, entry_id.1
                );
                Ok(player)
            }
            Err(err) => {
                error!(
                    "Failed to update player with id: {} in game: {}. \n With error: {}",
                    entry_id.0, entry_id.1, err
                );
                Err(CustomError::InternalServerError(None))
            }
        }
    }

    fn fetch_entry(&mut self, entry_id: PlayerPrimaryKey) -> Result<Player, CustomError> {
        match players::table
            .filter(players::player_id.eq(entry_id.0.clone()))
            .filter(players::game_id.eq(entry_id.1))
            .first(&mut self.database_connection)
        {
            Ok(player) => {
                info!(
                    "Fetched player with id: {} in game: {}",
                    entry_id.0, entry_id.1
                );
                Ok(player)
            }
            Err(err) => {
                error!(
                    "Failed to fetch player with id: {} in game: {}. \n With error: {}",
                    entry_id.0, entry_id.1, err
                );
                Err(CustomError::InternalServerError(None))
            }
        }
    }

    fn fetch_entries(
        &mut self,
        request_options: RequestOptions<i32>,
    ) -> Result<Vec<Player>, CustomError> {
        let limit = match request_options.limit {
            Some(limit) => limit,
            None => 10,
        };

        let offset = match request_options.page {
            Some(page) => calculate_offset(limit, page),
            None => calculate_offset(limit, 0),
        };

        match request_options.filter_value {
            Some(game_id) => {
                match players::table
                    .filter(players::game_id.eq(game_id))
                    .limit(limit)
                    .offset(offset)
                    .load(&mut self.database_connection)
                {
                    Ok(players) => {
                        info!("Fetched {} players", players.len());
                        Ok(players)
                    }
                    Err(err) => {
                        error!("Failed to fetch players. \n With error: {}", err);
                        Err(CustomError::InternalServerError(None))
                    }
                }
            }
            None => {
                match players::table
                    .limit(limit)
                    .offset(offset)
                    .load(&mut self.database_connection)
                {
                    Ok(players) => {
                        info!("Fetched {} players", players.len());
                        Ok(players)
                    }
                    Err(err) => {
                        error!("Failed to fetch players. \n With error: {}", err);
                        Err(CustomError::InternalServerError(None))
                    }
                }
            }
        }
    }
}
