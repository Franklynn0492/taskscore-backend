use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::repository::repository::Repository;
use crate::model::{MessageResponder, User, Session};

#[openapi]
#[get("/user/<id>")]
pub fn get_user<'a>(id: u32, repository: &State<Repository>) -> Option<Json<User>> {
    // Option.map_or() returns a default value if the option is None, or otherwise applies a function to the in the Some(x) contained value, returning an option
    repository.get_user(id).map_or(None, |user| Some(Json(user)))
}

#[openapi]
#[get("/user")]
pub fn get_current_user<'a>(session: Session) -> Json<User> {
    Json(session.user.lock().unwrap().clone())
}

#[openapi]
#[get("/user/all")]
pub fn get_all_users<'a>(repository: &State<Repository>) -> Json<Vec<User>> {
    Json(repository.get_all_users())
}

#[openapi]
#[post("/user")]
pub fn add_user<'a>(session: Session, user: User, repository: &State<Repository>) -> MessageResponder<u32> {
    repository.add_user(&session, user)
}