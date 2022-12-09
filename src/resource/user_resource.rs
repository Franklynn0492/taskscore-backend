use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::{model::{User, Session}, logic::logic::{Logic, ApplicationLogic}};

use super::http::responder::MessageResponder;

#[openapi(tag = "User")]
#[get("/user/<id>")]
pub async fn get_user<'a>(id: u32, repository: &State<ApplicationLogic>) -> Option<Json<User>> {
    // Option.map_or() returns a default value if the option is None, or otherwise applies a function to the in the Some(x) contained value, returning an option
    repository.get_user(id).await.map_or(None, |user| Some(Json(user)))
}

#[openapi(tag = "User")]
#[get("/user/username/<username>")]
pub async fn get_user_by_username<'a>(username: String, repository: &State<ApplicationLogic>) -> Option<Json<User>> {
    repository.find_user_by_username_const(&username).await.map_or(None, |user| Some(Json(user)))
}

#[openapi(tag = "User")]
#[get("/user")]
pub async fn get_current_user<'a>(session: Session) -> Json<User> {
    Json(session.user.lock().unwrap().clone())
}

#[openapi(tag = "User")]
#[get("/user/all")]
pub async fn get_all_users<'a>(repository: &State<ApplicationLogic>) -> Json<Vec<User>> {
    Json(repository.get_all_users().await)
}

#[openapi(tag = "User")]
#[post("/user")]
pub async fn add_user<'a>(session: Session, user: User, repository: &State<ApplicationLogic>) -> MessageResponder<u32> {
    repository.add_user(&session, user).await
}