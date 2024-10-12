#[macro_use] extern crate rocket;
use rocket_dyn_templates::{Template};
use rocket::fs::FileServer;
mod controllers;


#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Template::fairing())
    .mount("/static",FileServer::from("src/views/static"))
    .mount("/", routes![controllers::home_controller::home_get])
}
