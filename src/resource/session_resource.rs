use rocket::http::{Cookie, CookieJar};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::logic::logic::{Logic, ApplicationLogic};
use crate::model::session::LoginRequest;
use crate::model::{Session};

#[openapi(tag = "Session")]
#[post("/session/login")]
pub async fn login<'a>(login_request: LoginRequest, repository: &State<ApplicationLogic>, jar: &CookieJar<'_>) -> Result<Json<Session>, NotFound<String>> {
    let session_result = repository.login(login_request).await;
    match session_result {
        Ok(session) => {
            let session_id = session.id.unwrap().to_string();
            jar.add(Cookie::new("sid", session_id).into_owned());
            Ok(Json(session))
        },
        Err(error) => Err(NotFound(error))
    }
}

#[openapi(tag = "Session")]
#[get("/session")]
pub async fn get_current_session<'a>(session: Session) -> Json<Session> {
    Json(session)
}

#[openapi(tag = "Session")]
#[delete("/session/logout")]
pub async fn logout<'a>(session: Session, repository: &State<ApplicationLogic>) -> Result<Json<()>, NotFound<String>> {
    match repository.logout(&session.session_id).await {
        Ok(_) => Ok(Json(())),
        Err(msg) => Err(NotFound(msg))
    }
}