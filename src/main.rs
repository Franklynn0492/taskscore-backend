use repository::neo4j_repsitory::Neo4JRepository;
use futures::executor::block_on;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use std::env;

use resource::config_resource::*;
use resource::score_resource::*;
use resource::session_resource::*;
use resource::task_resource::*;
use resource::user_resource::*;
use resource::response::Response;
use rocket_okapi::{openapi, openapi_get_routes};


mod model;
mod repository;
mod resource;
mod logic;
mod client;

#[macro_use] extern crate rocket;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

#[openapi]
#[get("/")]
fn hello() -> Json<Response<String>> {
    Json(Response{data: "Hello, world!".to_owned()})
}

#[catch(404)]
fn not_found() -> NotFound<()> {
    NotFound(())
}

#[rocket::main]
async fn main() {
    print!("Starting TaskScore application. Database address: {}\n", env::var("TS_DATABASE_ADDRESS").unwrap_or("N/A".to_owned()));
    let context_root = "/rest";

    let _ = rocket::build()

    .manage(block_on(Neo4JRepository::connect()).unwrap())
    .mount(context_root, openapi_get_routes![hello,
        get_config,
        score, get_score_of_user, get_score_of_current_user,
        login, get_current_session, logout,
        get_user, get_current_user, get_all_users, add_user, get_user_by_username,
        get_task, get_all_tasks])
//    .mount(context_root, routes![])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}