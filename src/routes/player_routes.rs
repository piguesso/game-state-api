#[post("/<game_id>/users")]
pub fn join_game(game_id: String) -> String {
    format!("Hello, world! {}", game_id)
}

#[delete("/<game_id>/users/<user_id>")]
pub fn leave_game(game_id: String, user_id: String) -> String {
    format!("Hello, world! {} {}", game_id, user_id)
}

#[get("/<game_id>/users")]
pub fn get_users(game_id: String) -> String {
    format!("Hello, world! {}", game_id)
}
