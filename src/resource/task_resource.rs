use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::logic::logic::{Logic, ApplicationLogic};
use crate::model::{Task};

#[openapi(tag = "Task")]
#[get("/task/<id>")]
pub async fn get_task<'a>(id: u32, repository: &State<ApplicationLogic>) -> Option<Json<Task>> {
    repository.get_task(id).await.map_or(None, |task| Some(Json(task)))
}

#[openapi(tag = "Task")]
#[get("/task/all")]
pub async fn get_all_tasks<'a>(repository: &State<ApplicationLogic>) -> Json<Vec<Task>> {
    Json(repository.get_all_tasks().await)
}