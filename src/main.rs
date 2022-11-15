use repository::repository::Repository;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;

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
    let context_root = "/TaskScore/rest";

    let _ = rocket::build()

    .manage(Repository::init_repository())
    .mount(context_root, openapi_get_routes![hello,
        get_config,
        score, get_score_of_user, get_score_of_current_user,
        login, get_current_session, logout,
        get_user, get_current_user, get_all_users, add_user,
        get_task, get_all_tasks])
//    .mount(context_root, routes![])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}