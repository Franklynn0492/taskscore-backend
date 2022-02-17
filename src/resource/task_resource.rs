use rocket::serde::json::Json;
use rocket::State;
use crate::model::{Task};
use crate::repository::neo4j_repsitory::Neo4JRepository;
use crate::repository::repository::Repository;

#[get("/task/<id>")]
pub async fn get_task<'a>(id: u32, repository: &'a State<Neo4JRepository>) -> Option<Json<Task>> {
    repository.get_task(id).await.map_or(None, |task| Some(Json(task)))
}

#[get("/task/all")]
pub async fn get_all_tasks<'a>(repository: &'a State<Neo4JRepository>) -> Json<Vec<Task>> {
    Json(repository.get_all_tasks().await)
}