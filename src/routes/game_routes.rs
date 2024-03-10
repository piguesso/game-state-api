#[post("/")]
pub fn create_game() -> String {
    String::from("Hello, world!")
}

#[post("/", rank = 2)]
pub fn create_game_unauthorized() -> String {
    String::from("Hello, world!")
}

#[get("/<game_id>")]
pub fn get_game(game_id: String) -> String {
    format!("Hello, world! {}", game_id)
}

#[get("/?<status>&<page>&<limit>&<order_by>&<order>")]
pub fn get_games(
    status: Option<Vec<String>>,
    page: Option<u8>,
    limit: Option<u8>,
    order_by: Option<String>,
    order: Option<String>,
) -> String {
    String::from("Hello, world!")
}

#[delete("/<game_id>")]
pub fn delete_game(game_id: String) -> String {
    format!("Hello, world! {}", game_id)
}

#[put("/<game_id>")]
pub fn change_game_state(game_id: String) -> String {
    format!("Hello, world! {}", game_id)
}
