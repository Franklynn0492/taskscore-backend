use rocket::http::{Cookie, CookieJar};
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::State;
use rocket_okapi::openapi;
use crate::model::session::LoginRequest;
use crate::model::{Session};
use crate::repository::neo4j_repsitory::Neo4JRepository;
use crate::repository::repository::Repository;

#[openapi]
#[post("/session/login")]
pub async fn login<'a>(login_request: LoginRequest<'_>, repository: &State<Neo4JRepository>, jar: &CookieJar<'_>) -> Result<Json<Session>, NotFound<String>> {
    let session_result = repository.login(login_request).await;
    match session_result {
        Ok(session) => {
            let session_id: &str = session.id.as_str();
            jar.add(Cookie::new("sid", session_id).into_owned());
            Ok(Json(session))
        },
        Err(error) => Err(NotFound(error))
    }
}

#[openapi]
#[get("/session")]
pub async fn get_current_session<'a>(session: Session) -> Json<Session> {
    Json(session)
}

#[openapi]
#[delete("/session/logout")]
pub async fn logout<'a>(session: Session, repository: &State<Neo4JRepository>) -> Result<Json<()>, NotFound<String>> {
    match repository.logout(&session.id).await {
        Ok(_) => Ok(Json(())),
        Err(msg) => Err(NotFound(msg))
    }
}