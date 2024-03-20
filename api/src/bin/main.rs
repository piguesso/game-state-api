use rocket::{http::Status, response::{content, status}};

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> status::Custom<content::RawJson<String>> {
    let game = service::game_service::get_game(Some(1), None);
    match game {
        Ok(game) => {
            let final_game = serde_json::to_string(&game).unwrap();
            status::Custom(Status::Ok, content::RawJson(final_game))
        },
        Err(e) => {
            let result = serde_json::to_string(&e).unwrap();
            status::Custom(Status::BadRequest, content::RawJson(result))
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}