mod game_routes;
mod player_routes;

use self::game_routes::{
    change_game_state, create_game, create_game_unauthorized, delete_game, get_game, get_games,
};
use self::player_routes::{get_users, join_game, leave_game};

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_game,
        create_game_unauthorized,
        join_game,
        leave_game,
        get_users,
        get_game,
        get_games,
        delete_game,
        change_game_state
    ]
}
