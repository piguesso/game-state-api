#[macro_use]
extern crate rocket;

// Module imports
mod controller;
mod database;
mod error;
mod events;
mod repository;
mod secrets;
mod services;
mod utils;

use std::{borrow::Borrow, sync::Mutex};

// Use imports
use controller::alternative_game_stream;
use diesel::{
    r2d2::{self, ConnectionManager, Pool},
    PgConnection,
};
use repository::{
    new_game_repository, new_player_repository, new_player_round_scoring_repository,
    new_player_scoring_repository, new_round_repository,
};
use secrets::{ISecretProvider, SecretProvider};
use services::game_service::new_game_service;

fn new_connection_manager(provider: &SecretProvider) -> Pool<ConnectionManager<PgConnection>> {
    let database_url = provider.get_secret("DATABASE_CONNECTION_STRING").unwrap();
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager).unwrap()
}

#[launch]
fn rocket() -> _ {
    let secret_provider = SecretProvider::new(".env");
    let connection_manager = new_connection_manager(secret_provider.borrow());

    //let game_repository = new_game_repository(connection_manager.get().unwrap());
    let player_scoring_repository =
        new_player_scoring_repository(connection_manager.get().unwrap());
    let player_repository = new_player_repository(connection_manager.get().unwrap());
    let player_round_scoring_repository =
        new_player_round_scoring_repository(connection_manager.get().unwrap());
    let round_repository = new_round_repository(connection_manager.get().unwrap());

    //let game_service = new_game_service(Box::new(game_repository));

    rocket::build()
        .manage(secret_provider)
        .manage(Mutex::new(player_scoring_repository))
        .manage(Mutex::new(player_repository))
        .manage(Mutex::new(player_round_scoring_repository))
        .manage(Mutex::new(round_repository))
        .mount("/", routes![alternative_game_stream])
}
