use rocket::serde::json::Json;
use rocket::State;
use crate::repository::legacy_repository::Repository;
use crate::model::{Task};

#[get("/task/<id>")]
pub fn get_task<'a>(id: u32, repository: &'a State<Repository>) -> Option<Json<Task>> {
    repository.get_task(id).map_or(None, |task| Some(Json(task)))
}

#[get("/task/all")]
pub fn get_all_tasks<'a>(repository: &'a State<Repository>) -> Json<Vec<Task>> {
    Json(repository.get_all_tasks())
}