use repository::neo4j_repsitory::Neo4JRepository;
use rocket::futures::executor::block_on;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

use resource::config_resource::*;
use resource::score_resource::*;
use resource::session_resource::*;
use resource::task_resource::*;
use resource::user_resource::*;

mod model;
mod repository;
mod resource;

#[macro_use] extern crate rocket;

#[cfg(test)]
#[macro_use]
extern crate lazy_static;
extern crate dotenv;

#[get("/")]
fn hello() -> Json<String> {
    Json("Hello, world!".to_owned())
}

#[catch(404)]
fn not_found() -> NotFound<()> {
    NotFound(())
}

#[rocket::main]
async fn main() {
    let context_root = "/TaskScore/rest";

    let _ = rocket::build()

    .manage(block_on(Neo4JRepository::connect()).unwrap())
    .mount(context_root, routes![hello,
        get_user, get_current_user, get_all_users, add_user, get_user_by_username,
        get_task, get_all_tasks,
        score, get_score_of_user, get_score_of_current_user,
        get_config,
        login, get_current_session, logout])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}