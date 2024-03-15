use diesel::{data_types::PgTimestamp, prelude::*};

#[derive(Queryable)]
#[diesel(table_name = crate::database::schema::games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Game {
    pub id: i32,
    pub status: String,
    pub winner_id: Option<String>,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewGame {
    pub status: String,
    pub winner_id: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::database::schema::games)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateGame {
    pub status: String,
    pub winner_id: Option<String>,
}

#[derive(Queryable)]
#[diesel(table_name = crate::database::schema::players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(player_id, game_id))]
pub struct Player {
    pub player_id: String,
    pub game_id: i32,
    pub is_host: bool,
    pub left_game_at: Option<PgTimestamp>,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPlayer {
    pub player_id: String,
    pub game_id: i32,
    pub is_host: Option<bool>,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::database::schema::players)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(player_id, game_id))]
pub struct UpdatePlayer {
    pub player_id: String,
    pub game_id: i32,
    pub is_host: Option<bool>,
    pub left_game_at: Option<PgTimestamp>,
}

#[derive(Queryable)]
#[diesel(table_name = crate::database::schema::player_scoring)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(player_id))]
pub struct PlayerScoring {
    pub player_id: String,
    pub total_xp: i32,
    pub highest_score_game: i32,
    pub highest_score_round: i32,
    pub games_played: i32,
    pub games_won: i32,
    pub games_lost: i32,
    pub games_top3: i32,
    pub games_bottom3: i32,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::database::schema::player_scoring)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(player_id))]
pub struct UpdatePlayerScoring {
    pub player_id: String,
    pub total_xp: Option<i32>,
    pub highest_score_game: Option<i32>,
    pub highest_score_round: Option<i32>,
    pub games_played: Option<i32>,
    pub games_won: Option<i32>,
    pub games_lost: Option<i32>,
    pub games_top3: Option<i32>,
    pub games_bottom3: Option<i32>,
}

#[derive(Queryable, Selectable, Insertable, AsChangeset)]
#[diesel(table_name = crate::database::schema::rounds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Round {
    pub id: i32,
    pub game_id: i32,
    pub round_number: i32,
    pub topic: String,
    pub start_time: PgTimestamp,
    pub end_time: PgTimestamp,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::rounds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewRound {
    pub game_id: i32,
    pub round_number: i32,
    pub topic: String,
    pub start_time: PgTimestamp,
    pub end_time: PgTimestamp,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::database::schema::rounds)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateRound {
    pub id: i32,
    pub game_id: Option<i32>,
    pub round_number: Option<i32>,
    pub topic: Option<String>,
    pub start_time: Option<PgTimestamp>,
    pub end_time: Option<PgTimestamp>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::player_scoring_round)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewPlayerScoringRound {
    pub player_id: String,
    pub game_id: i32,
    pub round_id: i32,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::database::schema::player_scoring_round)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdatePlayerScoringRound {
    pub player_id: String,
    pub game_id: i32,
    pub round_id: i32,
    pub score: Option<i32>,
    pub place: Option<i32>,
    pub is_winner: Option<bool>,
    pub time_used_to_complete: Option<i64>,
    pub first_topic: Option<String>,
    pub second_topic: Option<String>,
    pub third_topic: Option<String>,
    pub has_stopped_game: Option<bool>,
}

#[derive(Queryable)]
#[diesel(table_name = crate::database::schema::player_scoring_round)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PlayerScoringRound {
    pub player_id: String,
    pub game_id: i32,
    pub round_id: i32,
    pub score: i32,
    pub place: i32,
    pub is_winner: bool,
    pub time_used_to_complete: i64,
    pub first_topic: String,
    pub second_topic: String,
    pub third_topic: String,
    pub has_stopped_game: bool,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
}
