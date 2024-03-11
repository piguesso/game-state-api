#[macro_use] extern crate rocket;

// Module imports
mod secrets;

// Use imports
use secrets::ISecretProvider;

#[get("/")]
fn index() -> String {
    String::from("Hello, world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
