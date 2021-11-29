

use model::{Score, Session, User};
use model::session::LoginRequest;
use model::task::Task;
use repository::Repository;
use rocket::response::status::Conflict;
use rocket::serde::json::Json;
use rocket::State;
use rocket::http::{Cookie, CookieJar};


mod model;
mod repository;

#[macro_use] extern crate rocket;

#[get("/")]
fn hello() -> Json<String> {
    Json("Hello, world!".to_owned())
}

#[get("/user/<id>")]
fn get_user<'a>(id: u32, repository: &'a State<Repository>) -> Json<Option<User>> {
    Json(repository.get_user(id))
}

#[get("/user")]
fn get_current_user<'a>(session: Session) -> Json<User> {
    Json(session.user.lock().unwrap().clone())
}

#[get("/user/all")]
fn get_all_users<'a>(repository: &'a State<Repository>) -> Json<Vec<User>> {
    Json(repository.get_all_users())
}

#[post("/user")]
fn add_user<'a>(user: User, repository: &'a State<Repository>) -> Result<Json<u32>, Conflict<String>> {
    let add_result = repository.add_user(user);
    match add_result {
        Ok(id) => Ok(Json(id)),
        Err(message) => Err(Conflict(Some(message)))
    }
}

#[get("/task/<id>")]
fn get_task<'a>(id: u32, repository: &'a State<Repository>) -> Json<Option<&'a Task>> {
    Json(repository.get_task(id))
}

#[get("/task/all")]
fn get_all_tasks<'a>(repository: &'a State<Repository>) -> Json<&'a Vec<Task>> {
    Json(repository.get_all_tasks())
}

#[post("/score/<task_id>")]
fn score<'a>(session: Session, task_id: u32, repository: &'a State<Repository>) -> Json<Result<u16, String>> {
    let user_mutex_guard = session.user.lock().unwrap();
    let user_id = user_mutex_guard.id;
    std::mem::drop(user_mutex_guard);
    Json(repository.score(user_id, task_id))
}

#[get("/score/<user_id>")]
fn get_score_of_user<'a>(user_id: u32, repository: &'a State<Repository>) -> Json<Option<Vec<Score>>> {
    Json(repository.get_user(user_id).and_then(|user| Some(user.scores)))
}

#[get("/score")]
fn get_score_of_current_user<'a>(session: Session) -> Json<Vec<Score>> {
    Json(session.user.lock().unwrap().clone().scores)
}

#[post("/session/login")]
fn login<'a>(login_request: LoginRequest, repository: &'a State<Repository>, jar: &'a CookieJar<'_>) -> Json<Result<Session, String>> {
    let session_result = repository.login(login_request);
    if session_result.is_ok() {
        let session_id: &str = session_result.as_ref().unwrap().id.as_str();
        jar.add(Cookie::new("sid", session_id).into_owned());
    }
    Json(session_result)
}

#[get("/session")]
fn get_current_session<'a>(session: Session) -> Json<Session> {
    Json(session)
}

#[delete("/session/logout")]
fn logout<'a>(session: Session, repository: &'a State<Repository>) -> Json<Result<(), String>> {
    Json(repository.logout(&session.id))
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("Route not found")
}

#[rocket::main]
async fn main() {
    let context_root = "/TaskScore/rest";

    let _ = rocket::build()

    .manage(Repository::init_repository())
    .mount(context_root, routes![hello,
        get_user, get_current_user, get_all_users, add_user,
        get_task, get_all_tasks,
        score, get_score_of_user, get_score_of_current_user,
        login, get_current_session, logout])
    .register(context_root, catchers![not_found])
    .launch()
    .await;
}