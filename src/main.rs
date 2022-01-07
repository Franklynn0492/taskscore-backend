

use model::{MessageResponder, Score, Session, User};
use model::session::LoginRequest;
use model::task::Task;
use repository::Repository;
use rocket::response::status::NotFound;
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
fn get_user<'a>(id: u32, repository: &'a State<Repository>) -> Option<Json<User>> {
    // Option.map_or() returns a default value if the option is None, or otherwise applies a function to the in the Some(x) contained value, returning an option
    repository.get_user(id).map_or(None, |user| Some(Json(user)))
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
fn add_user<'a>(session: Session, user: User, repository: &'a State<Repository>) -> MessageResponder<u32> {
    repository.add_user(&session, user)
}

#[get("/task/<id>")]
fn get_task<'a>(id: u32, repository: &'a State<Repository>) -> Option<Json<Task>> {
    repository.get_task(id).map_or(None, |task| Some(Json(task)))
}

#[get("/task/all")]
fn get_all_tasks<'a>(repository: &'a State<Repository>) -> Json<Vec<Task>> {
    Json(repository.get_all_tasks())
}

#[post("/score/<task_id>")]
fn score<'a>(session: Session, task_id: u32, repository: &'a State<Repository>) -> Result<Json<u16>, NotFound<String>> {
    let user_mutex_guard = session.user.lock().unwrap();
    let user_id = user_mutex_guard.id;
    std::mem::drop(user_mutex_guard);

    match repository.score(user_id, task_id) {
        Ok(new_score) => Ok(Json(new_score)),
        Err(msg) => Err(NotFound(msg))
    }
}

#[get("/score/<user_id>")]
fn get_score_of_user<'a>(user_id: u32, repository: &'a State<Repository>) -> Option<Json<Vec<Score>>> {
    repository.get_user(user_id).and_then(|user| Some(Json(user.scores)))
}

#[get("/score")]
fn get_score_of_current_user<'a>(session: Session) -> Json<Vec<Score>> {
    Json(session.user.lock().unwrap().clone().scores)
}

#[post("/session/login")]
fn login<'a>(login_request: LoginRequest, repository: &'a State<Repository>, jar: &'a CookieJar<'_>) -> Result<Json<Session>, NotFound<String>> {
    let session_result = repository.login(login_request);
    match session_result {
        Ok(session) => {
            let session_id: &str = session.id.as_str();
            jar.add(Cookie::new("sid", session_id).into_owned());
            Ok(Json(session))
        },
        Err(error) => Err(NotFound(error))
    }
}

#[get("/session")]
fn get_current_session<'a>(session: Session) -> Json<Session> {
    Json(session)
}

#[delete("/session/logout")]
fn logout<'a>(session: Session, repository: &'a State<Repository>) -> Result<Json<()>, NotFound<String>> {
    match repository.logout(&session.id) {
        Ok(_) => Ok(Json(())),
        Err(msg) => Err(NotFound(msg))
    }
}

#[catch(404)]
fn not_found() -> NotFound<()> {
    NotFound(())
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