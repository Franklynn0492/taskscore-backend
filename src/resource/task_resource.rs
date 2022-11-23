use rocket::http::hyper::server::accept;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::repository::repository::Repository;
use crate::model::{Task};
use crate::repository::neo4j_repsitory::Neo4JRepository;

#[openapi]
#[get("/task/<id>")]
pub async fn get_task<'a>(id: u32, repository: &State<Neo4JRepository>) -> Option<Json<Task>> {
    repository.get_task(id).await.map_or(None, |task| Some(Json(task)))
}

#[openapi]
#[get("/task/all")]
pub async fn get_all_tasks<'a>(repository: &State<Neo4JRepository>) -> Json<Vec<Task>> {
    Json(repository.get_all_tasks().await)
}