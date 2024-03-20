use chrono::NaiveDateTime;
use diesel::result::Error as DieselError;
use diesel::{prelude::*, update};
use infrastructure::establish_redis_connection;
use infrastructure::models::{NewRound, Player, Round, UpdatePlayerScoringRound, UpdateRound};
use infrastructure::schema::{games, player_scoring_round, players, rounds};
use infrastructure::{establish_connection, models::Game};
use redis::Commands;
use rocket::http::Status;
use shared::score::calculate_score;
use shared::{error::Error, game_status::GameStatus};

pub fn change_to_playing(game_id: i32, requester_id: String) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game: Result<Game, DieselError> = games::table
        .select(Game::as_select())
        .find(game_id)
        .first::<Game>(&mut conn);
    match game {
        Ok(game) => {
            if game.status == Some(GameStatus::to_string(GameStatus::WAITING)) {
                let player: Player = match players::table
                    .select(Player::as_select())
                    .filter(players::player_id.eq(requester_id))
                    .filter(players::game_id.eq(game_id))
                    .first::<Player>(&mut conn)
                {
                    Ok(player) => player,
                    Err(diesel::NotFound) => {
                        return Err(Error::new(
                            String::from("Player not found"),
                            Status::NotFound,
                        ))
                    }
                    Err(_) => {
                        return Err(Error::new(
                            String::from("Internal server error"),
                            Status::InternalServerError,
                        ))
                    }
                };

                if player.is_host == Some(false) {
                    return Err(Error::new(
                        String::from("Only the host can start the game"),
                        Status::BadRequest,
                    ));
                } else {
                    let mut redis_conn = establish_redis_connection();
                    match redis_conn.set::<String, String, bool>(
                        format!("game:{}:status", game_id),
                        GameStatus::to_string(GameStatus::PLAYING),
                    ) {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Error::new(
                            String::from("Internal server error"),
                            Status::InternalServerError,
                        )),
                    }
                }
            } else {
                Err(Error::new(
                    String::from("Game is not in the correct status to be played"),
                    Status::BadRequest,
                ))
            }
        }
        Err(diesel::NotFound) => Err(Error::new(String::from("Game not found"), Status::NotFound)),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn change_to_finished(game_id: i32, requester_id: String) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game: Game = match games::table
        .select(Game::as_select())
        .find(game_id)
        .first::<Game>(&mut conn)
    {
        Ok(game) => game,
        Err(diesel::NotFound) => {
            return Err(Error::new(String::from("Game not found"), Status::NotFound))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if game.status == Some(GameStatus::to_string(GameStatus::FINISHED)) {
        return Err(Error::new(
            String::from("Game is already finished"),
            Status::BadRequest,
        ));
    }

    let player: Player = match players::table
        .select(Player::as_select())
        .filter(players::player_id.eq(requester_id))
        .filter(players::game_id.eq(game_id))
        .first::<Player>(&mut conn)
    {
        Ok(player) => player,
        Err(diesel::NotFound) => {
            return Err(Error::new(
                String::from("Player not found"),
                Status::NotFound,
            ))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if player.is_host == Some(false) {
        return Err(Error::new(
            String::from("Only the host can finish the game"),
            Status::BadRequest,
        ));
    } else {
        let mut redis_conn = establish_redis_connection();
        match redis_conn.del::<String, bool>(format!("game:{}:status", game_id)) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            )),
        }
    }
}

pub fn start_new_round(game_id: i32, requester_id: String) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game: Game = match games::table
        .select(Game::as_select())
        .find(game_id)
        .first::<Game>(&mut conn)
    {
        Ok(game) => game,
        Err(diesel::NotFound) => {
            return Err(Error::new(String::from("Game not found"), Status::NotFound))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if game.status != Some(GameStatus::to_string(GameStatus::PLAYING)) {
        return Err(Error::new(
            String::from("Game is not in the correct status to start a new round"),
            Status::BadRequest,
        ));
    }

    let player: Player = match players::table
        .select(Player::as_select())
        .filter(players::player_id.eq(requester_id))
        .filter(players::game_id.eq(game_id))
        .first::<Player>(&mut conn)
    {
        Ok(player) => player,
        Err(diesel::NotFound) => {
            return Err(Error::new(
                String::from("Player not found"),
                Status::NotFound,
            ))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if player.is_host == Some(false) {
        return Err(Error::new(
            String::from("Only the host can start a new round"),
            Status::BadRequest,
        ));
    }

    let round_count: i64 = match rounds::table
        .count()
        .filter(rounds::game_id.eq(game_id))
        .first::<i64>(&mut conn)
    {
        Ok(round_count) => round_count,
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if (game.rounds as i64) == round_count {
        return Err(Error::new(
            String::from("All rounds have been played"),
            Status::BadRequest,
        ));
    }

    let mut redis_conn = establish_redis_connection();

    match redis_conn.incr::<String, i32, bool>(format!("game:{}:rounds", game_id), 1) {
        Ok(_) => (),
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    let new_round = NewRound {
        game_id,
        round_number: (round_count as i32) + 1,
        topic: String::from(""),
        start_time: chrono::Utc::now().naive_utc(),
    };

    match diesel::insert_into(rounds::table)
        .values(&new_round)
        .execute(&mut conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn finish_round(
    game_id: i32,
    requester_id: String,
    first_topic: String,
    second_topic: String,
    third_topic: String,
) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game: Game = match games::table
        .select(Game::as_select())
        .find(game_id)
        .first::<Game>(&mut conn)
    {
        Ok(game) => game,
        Err(diesel::NotFound) => {
            return Err(Error::new(String::from("Game not found"), Status::NotFound))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if game.status != Some(GameStatus::to_string(GameStatus::PLAYING)) {
        return Err(Error::new(
            String::from("Game is not in the correct status to finish a round"),
            Status::BadRequest,
        ));
    }

    match players::table
        .select(Player::as_select())
        .filter(players::player_id.eq(requester_id.clone()))
        .filter(players::game_id.eq(game_id))
        .first::<Player>(&mut conn)
    {
        Ok(player) => player,
        Err(diesel::NotFound) => {
            return Err(Error::new(
                String::from("Player not found"),
                Status::NotFound,
            ))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    let mut redis_conn = establish_redis_connection();

    let current_round: i32 = match redis_conn.get::<String, i32>(format!("game:{}:rounds", game_id))
    {
        Ok(round) => round,
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    let round: Round = match rounds::table
        .select(Round::as_select())
        .filter(rounds::game_id.eq(game_id))
        .filter(rounds::round_number.eq(current_round))
        .first::<Round>(&mut conn)
    {
        Ok(round) => round,
        Err(diesel::NotFound) => {
            return Err(Error::new(
                String::from("Round not found"),
                Status::NotFound,
            ))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if let Some(_) = round.end_time {
        return Err(Error::new(
            String::from("Round has already been finished"),
            Status::BadRequest,
        ));
    }

    let end_time = chrono::Utc::now().naive_utc();

    let updated_round = UpdateRound {
        end_time: Some(end_time),
    };

    match update(rounds::table)
        .filter(rounds::game_id.eq(game_id))
        .filter(rounds::round_number.eq(current_round))
        .set(updated_round)
        .execute(&mut conn)
    {
        Ok(_) => (),
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    let score = calculate_score(
        round.topic,
        first_topic.clone(),
        second_topic.clone(),
        third_topic.clone(),
        true,
    );

    let start_time_millis = round.start_time.and_utc().timestamp_millis();
    let end_time_millis = round.start_time.and_utc().timestamp_millis();

    let needed_time = end_time_millis - start_time_millis;

    let player_scoring = UpdatePlayerScoringRound {
        player_id: requester_id.clone(),
        game_id,
        round_id: current_round,
        time_used_to_complete: Some(needed_time),
        score: Some(score),
        place: None,
        is_winner: None,
        first_topic: Some(first_topic),
        second_topic: Some(second_topic),
        third_topic: Some(third_topic),
        has_stopped_game: Some(true),
    };

    match update(player_scoring_round::table)
        .filter(player_scoring_round::game_id.eq(game_id))
        .filter(player_scoring_round::player_id.eq(requester_id))
        .filter(player_scoring_round::round_id.eq(current_round))
        .set(player_scoring)
        .execute(&mut conn)
    {
        Ok(_) => (),
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    Ok(())
}

pub struct RoundResults {
    pub game_id: i32,
    pub player_id: String,
    pub first_topic: String,
    pub second_topic: String,
    pub third_topic: String,
    pub received: NaiveDateTime,
}

pub fn send_results(data: RoundResults) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game: Game = match games::table
        .select(Game::as_select())
        .find(data.game_id)
        .first::<Game>(&mut conn)
    {
        Ok(game) => game,
        Err(diesel::NotFound) => {
            return Err(Error::new(String::from("Game not found"), Status::NotFound))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    if game.status != Some(GameStatus::to_string(GameStatus::PLAYING)) {
        return Err(Error::new(
            String::from("Game is not in the correct status to send results"),
            Status::BadRequest,
        ));
    }

    let mut redis_conn = establish_redis_connection();

    match redis_conn.sismember(
        format!("game:{}:players", data.game_id),
        data.player_id.clone(),
    ) {
        Ok(true) => (),
        Ok(false) => {
            return Err(Error::new(
                String::from("Player is not in the game"),
                Status::BadRequest,
            ))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    let current_round: i32 =
        match redis_conn.get::<String, i32>(format!("game:{}:rounds", data.game_id)) {
            Ok(round) => round,
            Err(_) => {
                return Err(Error::new(
                    String::from("Internal server error"),
                    Status::InternalServerError,
                ))
            }
        };

    let round: Round = match rounds::table
        .select(Round::as_select())
        .filter(rounds::game_id.eq(data.game_id))
        .filter(rounds::round_number.eq(current_round))
        .first::<Round>(&mut conn)
    {
        Ok(round) => round,
        Err(diesel::NotFound) => {
            return Err(Error::new(
                String::from("Round not found"),
                Status::NotFound,
            ))
        }
        Err(_) => {
            return Err(Error::new(
                String::from("Internal server error"),
                Status::InternalServerError,
            ))
        }
    };

    let score = calculate_score(
        round.topic,
        data.first_topic.clone(),
        data.second_topic.clone(),
        data.third_topic.clone(),
        false,
    );

    let player_scoring = UpdatePlayerScoringRound {
        player_id: data.player_id.clone(),
        game_id: data.game_id,
        round_id: current_round,
        time_used_to_complete: None,
        score: Some(score),
        place: None,
        is_winner: None,
        first_topic: Some(data.first_topic),
        second_topic: Some(data.second_topic),
        third_topic: Some(data.third_topic),
        has_stopped_game: Some(false),
    };

    match update(player_scoring_round::table)
        .filter(player_scoring_round::game_id.eq(data.game_id))
        .filter(player_scoring_round::player_id.eq(data.player_id))
        .filter(player_scoring_round::round_id.eq(current_round))
        .set(player_scoring)
        .execute(&mut conn)
    {
        Ok(_) => Ok(()),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}
