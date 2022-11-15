use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::repository::repository::Repository;
use crate::model::{MessageResponder, User, Session};
use crate::repository::neo4j_repsitory::Neo4JRepository;

#[openapi]
#[get("/user/<id>")]
pub async fn get_user<'a>(id: u32, repository: &State<Neo4JRepository>) -> Option<Json<User>> {
    // Option.map_or() returns a default value if the option is None, or otherwise applies a function to the in the Some(x) contained value, returning an option
    repository.get_user(id).await.map_or(None, |user| Some(Json(user)))
}

#[openapi]
#[get("/user/username/<username>")]
pub async fn get_user_by_username<'a>(username: String, repository: &State<Neo4JRepository>) -> Option<Json<User>> {
    repository.find_user_by_username_const(&username).await.map_or(None, |user| Some(Json(user)))
}

#[openapi]
#[get("/user")]
pub async fn get_current_user<'a>(session: Session) -> Json<User> {
    Json(session.user.lock().unwrap().clone())
}

#[openapi]
#[get("/user/all")]
pub async fn get_all_users<'a>(repository: &State<Neo4JRepository>) -> Json<Vec<User>> {
    Json(repository.get_all_users().await)
}

#[openapi]
#[post("/user")]
pub async fn add_user<'a>(session: Session, user: User, repository: &State<Neo4JRepository>) -> MessageResponder<u32> {
    repository.add_user(&session, user).await
}