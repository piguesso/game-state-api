use diesel::{Connection, PgConnection};
use dotenvy::dotenv;
use redis::{Client, Connection as RedisConnection};
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&db_url).expect(&format!("Error connecting to {}", db_url))
}

pub fn establish_redis_connection() -> RedisConnection {
    dotenv().ok();
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let client = Client::open(redis_url).expect("Error connecting to Redis");
    client.get_connection().expect("Error connecting to Redis")
}
