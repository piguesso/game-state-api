use infrastructure::models::{Player, PlayerScoring, PlayerScoringRound};
use shared::error::Error;

// TODO
pub fn get_players_in_game(id: i32) -> Result<Vec<Player>, Error> {}

pub fn join_game(game_id: i32, player_id: String, role: Option<String>) -> Result<(), Error> {}

pub fn leave_game(game_id: i32, player_id: String) -> Result<(), Error> {}

pub fn get_player_stats_overall(game_id: i32, player_id: String) -> Result<PlayerScoring, Error> {}

pub fn get_player_stats_per_round(
    game_id: i32,
    round_id: i32,
    player_id: String,
) -> Result<PlayerScoringRound, Error> {
}

pub fn get_player_stats_overall_all(game_id: i32) -> Result<Vec<String>, Error> {}

pub fn get_player_stats_per_round_all(
    game_id: i32,
    round_id: i32,
) -> Result<Vec<PlayerScoringRound>, Error> {
}
