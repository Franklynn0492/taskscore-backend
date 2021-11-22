
use model::tasks::Task;
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

#[get("/user/all")]
fn get_all_users<'a>(repository: &'a State<Repository>) -> Json<&'a Vec<User>> {
    Json(repository.get_all_users())
}

#[get("/task/<id>")]
fn get_task<'a>(id: u32, repository: &'a State<Repository>) -> Json<Option<&'a Task>> {
    Json(repository.get_task(id))
}

#[get("/task/all")]
fn get_all_tasks<'a>(repository: &'a State<Repository>) -> Json<&'a Vec<Task>> {
    Json(repository.get_all_tasks())
}
/*
#[post("/score/<user_id>/<task_id>")]
fn score<'a>(user_id: u32, task_id: u32, repository: &'a mut State<Repository>) -> Json<Result<&'a User, String>> {
    Json(repository.score(user_id, task_id))
}*/

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("Route not found")
}

#[rocket::main]
async fn main() {
    let context_root = "/TaskScore/rest";

    let _ = rocket::build()

    .manage(Repository::init_repository())
    .mount(context_root, routes![hello, get_user, get_all_users, get_task, get_all_tasks])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}