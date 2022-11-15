use futures::executor::block_on;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::repository::repository::Repository;
use crate::model::{Score, Session};
use crate::repository::neo4j_repsitory::Neo4JRepository;

#[openapi]
#[post("/score/<task_id>")]
pub async fn score<'a>(session: Session, task_id: u32, repository: &State<Neo4JRepository>) -> Result<Json<u16>, NotFound<String>> {
    let user_mutex_guard = session.user.lock().unwrap();
    let user_id = user_mutex_guard.id;
    std::mem::drop(user_mutex_guard);

    match block_on(repository.score(user_id, task_id)) {
        Ok(new_score) => Ok(Json(new_score)),
        Err(msg) => Err(NotFound(msg))
    }
}

#[openapi]
#[get("/score/<user_id>")]
pub async fn get_score_of_user<'a>(user_id: u32, repository: &State<Neo4JRepository>) -> Option<Json<Vec<Score>>> {
    repository.get_user(user_id).await.and_then(|user| Some(Json(user.scores)))
}

#[openapi]
#[get("/score")]
pub async fn get_score_of_current_user<'a>(session: Session) -> Json<Vec<Score>> {
    Json(session.user.lock().unwrap().clone().scores)
}