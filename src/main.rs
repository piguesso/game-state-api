// Macro imports
#[macro_use]
extern crate rocket;

// Module imports
mod database;
mod error;
mod repositories;
mod routes;
mod secrets;
mod services;
mod types;
mod utils;
mod events;

// Use import
use diesel::r2d2::{self, ConnectionManager, Pool};
use diesel::PgConnection;
use redis::Connection as RedisConnection;
use repositories::new_state_handler;
use routes::routes;
use secrets::{new_source_provider, ISecretProvider};

fn connect_to_redis(provider: &impl ISecretProvider) -> RedisConnection {
    let redis_host = provider.get_secret(String::from("REDIS_HOST")).unwrap();
    let redis_port = provider.get_secret(String::from("REDIS_PORT")).unwrap();
    let redis_url = format!("redis://{}:{}", redis_host, redis_port);

    info!("Connecting to Redis at {}", redis_url.as_str());
    let client = redis::Client::open(redis_url).unwrap();
    let connection = client.get_connection().expect("Failed to connect to redis");

    info!("Connected to Redis");
    connection
}

fn new_connection_manager(
    provider: Box<dyn ISecretProvider>,
) -> Pool<ConnectionManager<PgConnection>> {
    let database_url = provider
        .get_secret(String::from("DATABASE_CONNECTION_STRING"))
        .unwrap();

    info!("Creating connection pool");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder().build(manager).unwrap();
    info!("Connection pool created");
    pool
}

fn init_logging() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
    info!("Logging initialized");
}

// Launch backend service
#[launch]
fn rocket() -> _ {
    init_logging();

    let provider = new_source_provider(String::from(".env"));

    let redis_connection = connect_to_redis(&provider);

    let redis_handler = new_state_handler(redis_connection);

    let connection_manager = new_connection_manager(Box::new(provider));

    let game_repository = repositories::new_game_repository(connection_manager.get().unwrap());
    info!("Game repository initialized");

    let round_repository = repositories::new_round_repository(connection_manager.get().unwrap());
    info!("Round repository initialized");

    let player_repository = repositories::new_player_repository(connection_manager.get().unwrap());
    info!("Player repository initialized");

    rocket::build().mount("/", routes())
}
