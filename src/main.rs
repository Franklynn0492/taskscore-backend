
use model::tasks::Task;
use repository::Repository;
use rocket::response::status::Conflict;
use rocket::serde::json::Json;
use rocket::State;

mod model;
mod repository;

use model::users::*;
use repository::*;

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> Json<String> {
    Json("Hello, world!".to_owned())
}

#[get("/user/<id>")]
fn get_user<'a>(id: u32, repository: &'a State<Repository>) -> Json<Option<User>> {
    Json(repository.get_user(id))
}

#[get("/user/all")]
fn get_all_users<'a>(repository: &'a State<Repository>) -> Json<Vec<User>> {
    Json(repository.get_all_users())
}

#[post("/user")]
fn add_user<'a>(user: User, repository: &'a State<Repository>) -> Result<Json<u32>, Conflict<String>> {
    let add_result = repository.add_user(user);
    match add_result {
        Ok(id) => Ok(Json(id)),
        Err(message) => Err(Conflict(Some(message)))
    }
}

#[get("/task/<id>")]
fn get_task<'a>(id: u32, repository: &'a State<Repository>) -> Json<Option<&'a Task>> {
    Json(repository.get_task(id))
}

#[get("/task/all")]
fn get_all_tasks<'a>(repository: &'a State<Repository>) -> Json<&'a Vec<Task>> {
    Json(repository.get_all_tasks())
}

#[post("/score/<user_id>/<task_id>")]
fn score<'a>(user_id: u32, task_id: u32, repository: &'a State<Repository>) -> Json<Result<u16, String>> {
    Json(repository.score(user_id, task_id))
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
    .mount(context_root, routes![hello, get_user, get_all_users, get_task, get_all_tasks, score, add_user])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}