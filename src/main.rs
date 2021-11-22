
use repository::Repository;
use rocket::serde::json::Json;
use rocket::State;

mod model;
mod repository;

use model::users::*;
use repository::*;

#[macro_use] extern crate rocket;
extern crate dotenv;

#[get("/")]
fn hello() -> Json<String> {
    Json("Hello, world!".to_owned())
}

#[get("/user/<id>")]
fn get_user<'a>(id: u32, repository: &'a State<Repository>) -> Json<Option<&'a User>> {
    Json(repository.get_user(id))
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("Route not found")
}

#[rocket::main]
async fn main() {
    let context_root = "/TaskScore/rest";

    let _ = rocket::build()

    .manage(Repository::init_repository())
    .mount(context_root, routes![hello, get_user])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}