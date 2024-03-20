use diesel::result::Error as DieselError;
use diesel::{prelude::*, update};
use infrastructure::models::UpdatePlayer;
use infrastructure::schema::player_scoring_round;
use infrastructure::{
    establish_connection, establish_redis_connection,
    models::{Game, Player, PlayerScoringRound},
    schema::{games, players},
};
use redis::Commands;
use rocket::http::Status;
use shared::error::Error;
use shared::response::{GamePlayerStats, PlayerRoundStats};

pub fn get_players_in_game(game_id: i32) -> Result<Vec<Player>, Error> {
    let mut conn = establish_connection();
    let game: Result<Game, DieselError> = games::table
        .select(Game::as_select())
        .find(game_id)
        .first::<Game>(&mut conn);
    match game {
        Ok(game) => {
            if game.status == Some(String::from("waiting"))
                || game.status == Some(String::from("playing"))
            {
                let mut redis_conn = establish_redis_connection();
                let player_ids = match redis_conn
                    .smembers::<String, Vec<String>>(format!("game:{}:players", game_id))
                {
                    Ok(player_ids) => player_ids,
                    Err(_) => {
                        return Err(Error::new(
                            String::from("Internal server error"),
                            Status::InternalServerError,
                        ))
                    }
                };

                let mut players = Vec::new();

                for player_id in player_ids {
                    let player = match players::table
                        .select(Player::as_select())
                        .filter(players::player_id.eq(player_id))
                        .first::<Player>(&mut conn)
                    {
                        Ok(player) => player,
                        Err(_) => {
                            return Err(Error::new(
                                String::from("Internal server error"),
                                Status::InternalServerError,
                            ))
                        }
                    };
                    players.push(player);
                }

                Ok(players)
            } else {
                Err(Error::new(
                    String::from("Game not in progress"),
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

pub fn join_game(game_id: i32, player_id: String) -> Result<(), Error> {
    let mut conn = establish_connection();
    let game: Result<Game, DieselError> = games::table
        .select(Game::as_select())
        .find(game_id)
        .first::<Game>(&mut conn);
    match game {
        Ok(game) => {
            if game.status == Some(String::from("waiting")) {
                let mut redis_conn = establish_redis_connection();
                match redis_conn
                    .sadd::<String, String, bool>(format!("game:{}:players", game_id), player_id)
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Error::new(
                        String::from("Internal server error"),
                        Status::InternalServerError,
                    )),
                }
            } else {
                Err(Error::new(
                    String::from("Game not in waiting state"),
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

pub fn leave_game(game_id: i32, player_id: String) -> Result<(), Error> {
    let mut redis_conn = establish_redis_connection();
    let is_member = redis_conn
        .sismember::<String, String, bool>(format!("game:{}:player", game_id), player_id.clone());
    match is_member {
        Ok(true) => {
            match redis_conn.srem::<String, String, bool>(
                format!("game:{}:players", game_id),
                player_id.clone(),
            ) {
                Ok(_) => {
                    let mut conn = establish_connection();
                    let updated_player = UpdatePlayer {
                        game_id,
                        is_host: None,
                        left_game_at: Some(chrono::Utc::now().naive_utc()),
                        player_id: player_id.clone(),
                    };
                    let result = update(players::table)
                        .filter(players::player_id.eq(player_id))
                        .set(updated_player)
                        .execute(&mut conn);
                    match result {
                        Ok(_) => Ok(()),
                        Err(_) => Err(Error::new(
                            String::from("Internal server error"),
                            Status::InternalServerError,
                        )),
                    }
                }
                Err(_) => Err(Error::new(
                    String::from("Internal server error"),
                    Status::InternalServerError,
                )),
            }
        }
        Ok(false) => Err(Error::new(
            String::from("Player not in game"),
            Status::BadRequest,
        )),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn get_player_stats_per_game(
    game_id: i32,
    player_id: String,
) -> Result<GamePlayerStats, Error> {
    let mut conn: PgConnection = establish_connection();
    let game: Result<Game, DieselError> = games::table
        .select(Game::as_select())
        .find(game_id)
        .first::<Game>(&mut conn);
    match game {
        Ok(_) => {
            let player_stats: Vec<PlayerScoringRound> = match player_scoring_round::table
                .select(PlayerScoringRound::as_select())
                .filter(player_scoring_round::game_id.eq(game_id))
                .filter(player_scoring_round::player_id.eq(player_id.clone()))
                .get_results::<PlayerScoringRound>(&mut conn)
            {
                Ok(player_stats) => player_stats,
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

            let mut round_stats = Vec::new();

            for stat in player_stats {
                let rnd_stats = PlayerRoundStats {
                    round_id: stat.round_id,
                    score: if stat.score == None {
                        0
                    } else {
                        stat.score.unwrap()
                    },
                    place: if stat.place == None {
                        0
                    } else {
                        stat.place.unwrap()
                    },
                    is_winner: if stat.is_winner == None {
                        false
                    } else {
                        stat.is_winner.unwrap()
                    },
                    time_used_to_complete: if stat.time_used_to_complete == None {
                        0
                    } else {
                        stat.time_used_to_complete.unwrap()
                    },
                    first_topic: if stat.first_topic == None {
                        String::from("")
                    } else {
                        stat.first_topic.unwrap()
                    },
                    second_topic: if stat.second_topic == None {
                        String::from("")
                    } else {
                        stat.second_topic.unwrap()
                    },
                    third_topic: if stat.third_topic == None {
                        String::from("")
                    } else {
                        stat.third_topic.unwrap()
                    },
                    has_stopped_game: if stat.has_stopped_game == None {
                        false
                    } else {
                        stat.has_stopped_game.unwrap()
                    },
                };
                round_stats.push(rnd_stats);
            }

            let overall_stats = GamePlayerStats {
                game_id,
                player_id,
                round_stats,
            };

            Ok(overall_stats)
        }
        Err(diesel::NotFound) => Err(Error::new(String::from("Game not found"), Status::NotFound)),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn get_player_stats_per_round(
    game_id: i32,
    round_id: i32,
    player_id: String,
) -> Result<PlayerScoringRound, Error> {
    let mut conn = establish_connection();
    let player_stats_round: Result<PlayerScoringRound, DieselError> = player_scoring_round::table
        .select(PlayerScoringRound::as_select())
        .filter(player_scoring_round::game_id.eq(game_id))
        .filter(player_scoring_round::round_id.eq(round_id))
        .filter(player_scoring_round::player_id.eq(player_id))
        .first::<PlayerScoringRound>(&mut conn);
    match player_stats_round {
        Ok(player_scoring) => Ok(player_scoring),
        Err(diesel::NotFound) => Err(Error::new(
            String::from("Player not found"),
            Status::NotFound,
        )),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn get_player_stats_per_game_all(game_id: i32) -> Result<Vec<GamePlayerStats>, Error> {
    let mut conn = establish_connection();
    let player_stats: Result<Vec<PlayerScoringRound>, DieselError> = player_scoring_round::table
        .select(PlayerScoringRound::as_select())
        .filter(player_scoring_round::game_id.eq(game_id))
        .get_results::<PlayerScoringRound>(&mut conn);
    match player_stats {
        Ok(player_stats) => {
            let mut game_player_stats = Vec::new();
            for stat in player_stats {
                let game_player_stat = GamePlayerStats {
                    game_id,
                    player_id: stat.player_id.clone(),
                    round_stats: vec![PlayerRoundStats {
                        round_id: stat.round_id,
                        score: if stat.score == None {
                            0
                        } else {
                            stat.score.unwrap()
                        },
                        place: if stat.place == None {
                            0
                        } else {
                            stat.place.unwrap()
                        },
                        is_winner: if stat.is_winner == None {
                            false
                        } else {
                            stat.is_winner.unwrap()
                        },
                        time_used_to_complete: if stat.time_used_to_complete == None {
                            0
                        } else {
                            stat.time_used_to_complete.unwrap()
                        },
                        first_topic: if stat.first_topic == None {
                            String::from("")
                        } else {
                            stat.first_topic.unwrap()
                        },
                        second_topic: if stat.second_topic == None {
                            String::from("")
                        } else {
                            stat.second_topic.unwrap()
                        },
                        third_topic: if stat.third_topic == None {
                            String::from("")
                        } else {
                            stat.third_topic.unwrap()
                        },
                        has_stopped_game: if stat.has_stopped_game == None {
                            false
                        } else {
                            stat.has_stopped_game.unwrap()
                        },
                    }],
                };
                game_player_stats.push(game_player_stat);
            }

            Ok(game_player_stats)
        }
        Err(diesel::NotFound) => Err(Error::new(String::from("Game not found"), Status::NotFound)),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}

pub fn get_player_stats_per_round_all(
    game_id: i32,
    round_id: i32,
) -> Result<Vec<PlayerScoringRound>, Error> {
    let mut conn = establish_connection();
    let player_stats: Result<Vec<PlayerScoringRound>, DieselError> = player_scoring_round::table
        .select(PlayerScoringRound::as_select())
        .filter(player_scoring_round::game_id.eq(game_id))
        .filter(player_scoring_round::round_id.eq(round_id))
        .get_results::<PlayerScoringRound>(&mut conn);
    match player_stats {
        Ok(player_stats) => Ok(player_stats),
        Err(diesel::NotFound) => Err(Error::new(String::from("Game not found"), Status::NotFound)),
        Err(_) => Err(Error::new(
            String::from("Internal server error"),
            Status::InternalServerError,
        )),
    }
}
