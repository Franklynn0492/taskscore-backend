use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use crate::repository::repository::Repository;
use crate::model::{Score, Session};

#[post("/score/<task_id>")]
pub fn score<'a>(session: Session, task_id: u32, repository: &'a State<Repository>) -> Result<Json<u16>, NotFound<String>> {
    let user_mutex_guard = session.user.lock().unwrap();
    let user_id = user_mutex_guard.id;
    std::mem::drop(user_mutex_guard);

    match repository.score(user_id, task_id) {
        Ok(new_score) => Ok(Json(new_score)),
        Err(msg) => Err(NotFound(msg))
    }
}

#[get("/score/<user_id>")]
pub fn get_score_of_user<'a>(user_id: u32, repository: &'a State<Repository>) -> Option<Json<Vec<Score>>> {
    repository.get_user(user_id).and_then(|user| Some(Json(user.scores)))
}

#[get("/score")]
pub fn get_score_of_current_user<'a>(session: Session) -> Json<Vec<Score>> {
    Json(session.user.lock().unwrap().clone().scores)
}