use infrastructure::{establish_connection, models::Game, schema::games};
use shared::{error::Error, game_status::GameStatus, RequestOptions};
use diesel::query_dsl::{RunQueryDsl, QueryDsl};

pub fn get_game(id: Option<i32>, slug: Option<String>) -> Result<Game, Error> {
    let mut conn = establish_connection();
    match id {
        Some(id) => {
            let game = games::table.find(id).first::<Game>(&mut conn);
            match game {
                Ok(game) => Ok(game),
                Err(e) => {
                    if e == diesel::NotFound {
                        return Err(Error::new(String::from("Game not found"), 404));
                    } else {
                        return Err(Error::new(String::from("Internal server error"), 500));
                    }
                }
            }
        },
        None => {
            match slug {
                Some(slug) => {
                    let game = games::table.filter(games::game_slug.eq(slug)).first::<Game>(&mut conn);
                    match game {
                        Ok(game) => Ok(game),
                        Err(e) => {
                            if e == diesel::NotFound {
                                return Err(Error::new(String::from("Game not found"), 404));
                            } else {
                                return Err(Error::new(String::from("Internal server error"), 500));
                            }
                        }
                    }
                },
                None => Err(Error::new(String::from("Invalid request"), 400))
            }
        }
    }
}

pub fn update_game(id: i32, status: Option<GameStatus>) -> Result<(), Error> {
    let mut conn = establish_connection();
}

// TODO
pub fn get_active_games(options: Option<RequestOptions>) -> Result<Vec<Game>, Error> {}
