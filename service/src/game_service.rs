use diesel::prelude::*;
use infrastructure::{
    establish_connection, establish_redis_connection,
    models::{Game, UpdateGame},
    schema::games,
};
use redis::Commands;
use rocket::http::Status;
use shared::{error::Error, game_status::GameStatus, RequestOptions};

pub fn get_game(id: Option<i32>, slug: Option<String>) -> Result<Game, Error> {
    let mut conn = establish_connection();
    match id {
        Some(id) => {
            let game = games::table
                .select(Game::as_select())
                .find(id)
                .first::<Game>(&mut conn);
            match game {
                Ok(game) => Ok(game),
                Err(diesel::NotFound) => {
                    Err(Error::new(String::from("Game not found"), Status::NotFound))
                }
                Err(_) => Err(Error::new(
                    String::from("Internal server error"),
                    Status::InternalServerError,
                )),
            }
        }
        None => match slug {
            Some(slug) => {
                let game = games::table
                    .select(Game::as_select())
                    .filter(games::game_slug.eq(slug))
                    .first::<Game>(&mut conn);
                match game {
                    Ok(game) => Ok(game),
                    Err(diesel::NotFound) => {
                        Err(Error::new(String::from("Game not found"), Status::NotFound))
                    }
                    Err(_) => Err(Error::new(
                        String::from("Internal server error"),
                        Status::InternalServerError,
                    )),
                }
            }
            None => Err(Error::new(
                String::from("Invalid request"),
                Status::BadRequest,
            )),
        },
    }
}

pub fn finish_game(id: i32, winner_id: String) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game = games::table
        .select(Game::as_select())
        .find(id)
        .first::<Game>(&mut conn);
    match game {
        Ok(_) => {
            let updated_game = UpdateGame {
                status: GameStatus::to_string(GameStatus::FINISHED),
                winner_id: Some(winner_id),
            };
            let result = diesel::update(games::table.find(id))
                .set(updated_game)
                .execute(&mut conn);
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::new(
                    String::from("Internal server error"),
                    Status::InternalServerError,
                )),
            }
        }
        Err(diesel::NotFound) => Err(Error::new(String::from("Game not found"), Status::NotFound)),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn end_game(id: i32) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game = games::table
        .select(Game::as_select())
        .find(id)
        .first::<Game>(&mut conn);
    match game {
        Ok(_) => {
            let updated_game = UpdateGame {
                status: GameStatus::to_string(GameStatus::FINISHED),
                winner_id: None,
            };
            let result = diesel::update(games::table.find(id))
                .set(updated_game)
                .execute(&mut conn);
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::new(
                    String::from("Internal server error"),
                    Status::InternalServerError,
                )),
            }
        }
        Err(diesel::NotFound) => Err(Error::new(String::from("Game not found"), Status::NotFound)),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn change_game_status(id: i32, status: GameStatus) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game = games::table
        .select(Game::as_select())
        .find(id)
        .first::<Game>(&mut conn);
    match game {
        Ok(_) => {
            let updated_game = UpdateGame {
                status: GameStatus::to_string(status),
                winner_id: None,
            };
            let result = diesel::update(games::table.find(id))
                .set(updated_game)
                .execute(&mut conn);
            match result {
                Ok(_) => Ok(()),
                Err(_) => Err(Error::new(
                    String::from("Internal server error"),
                    Status::InternalServerError,
                )),
            }
        }
        Err(diesel::NotFound) => Err(Error::new(String::from("Game not found"), Status::NotFound)),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn get_active_games(options: Option<RequestOptions>) -> Result<Vec<Game>, Error> {
    let mut redis_conn = establish_redis_connection();
    let mut conn = establish_connection();

    let active_games_ids = match redis_conn.smembers::<&str, Vec<i32>>("active_games") {
        Ok(active_games) => active_games,
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    let mut active_games = Vec::new();

    let opts = match options {
        Some(opts) => opts,
        None => RequestOptions {
            limit: 10,
            offset: 0,
        },
    };

    for game_id in active_games_ids {
        if active_games.len() == opts.limit as usize {
            break;
        }
        let game = games::table
            .select(Game::as_select())
            .filter(games::status.eq("waiting"))
            .or_filter(games::status.eq("playing"))
            .find(game_id)
            .first::<Game>(&mut conn);
        match game {
            Ok(game) => active_games.push(game),
            Err(diesel::NotFound) => continue,
            Err(_) => {
                return Err(Error::new(
                    String::from("Internal server error"),
                    Status::InternalServerError,
                ))
            }
        }
    }

    Ok(active_games)
}
