use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::repository::repository::Repository;
use crate::model::{Task};

#[openapi]
#[get("/task/<id>")]
pub fn get_task<'a>(id: u32, repository: &State<Repository>) -> Option<Json<Task>> {
    repository.get_task(id).map_or(None, |task| Some(Json(task)))
}

#[openapi]
#[get("/task/all")]
pub fn get_all_tasks<'a>(repository: &State<Repository>) -> Json<Vec<Task>> {
    Json(repository.get_all_tasks())
}