use rocket::serde::json::Json;
use rocket::State;
use crate::repository::legacy_repository::LegacyRepository;
use crate::model::{Task};
use crate::repository::repository::Repository;

#[get("/task/<id>")]
pub fn get_task<'a>(id: u32, repository: &'a State<LegacyRepository>) -> Option<Json<Task>> {
    repository.get_task(id).map_or(None, |task| Some(Json(task)))
}

#[get("/task/all")]
pub fn get_all_tasks<'a>(repository: &'a State<LegacyRepository>) -> Json<Vec<Task>> {
    Json(repository.get_all_tasks())
}