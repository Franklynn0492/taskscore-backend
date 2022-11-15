use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use rocket::{Request, http::Status, request::FromRequest, request::Outcome};
use rocket_okapi::{request::{OpenApiFromRequest, RequestHeaderInput}, gen::OpenApiGenerator};
use schemars::{JsonSchema, JsonSchema_repr};

use crate::repository::repository::Repository;

use super::User;
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const PASSWORD_LEN: usize = 30;

#[derive(serde::Serialize, Clone, OpenApiFromRequest, JsonSchema)]
pub struct Session {
    pub id: String,
    pub user: Arc<Mutex<User>>,
    pub started: DateTime::<Utc>,
    pub refreshed: DateTime::<Utc>,
}

impl Session {
    pub fn new(user: Arc<Mutex<User>>) -> Session {
        let now = Utc::now();
        Session {
            id: Session::generate_session_id(),
            user,
            started: now.clone(),
            refreshed: now,
        }
    }

    fn generate_session_id() -> String {
        let mut rng = rand::thread_rng();

        let session_id: String = (0..PASSWORD_LEN)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        session_id
    }

    fn refresh(&mut self) {
        self.refreshed = Utc::now();
    }
}

#[async_trait]
impl <'a> FromRequest<'a> for Session {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let repository = request.rocket().state::<Repository>();
        if repository.is_none() {
            return Outcome::Failure((Status::InternalServerError, "Missing status".to_owned()))
        }
        let repository = repository.unwrap();
        
        let cookie = request.cookies()
            .get("sid");
        if cookie.is_none() {
            return Outcome::Failure((Status::BadRequest, "No session provided".to_owned()))
        }
        let cookie = cookie.unwrap();

        let sid = cookie.value().to_owned();
        let session = repository.get_session(&sid);
        if session.is_none() {
            return Outcome::Failure((Status::Unauthorized, "Session not available".to_owned()))
        }

        let mut session = session.unwrap();
        session.refresh();
        Outcome::Success(session)
    }
}

#[derive(serde::Serialize, Clone)]
pub struct LoginRequest<'b> {
    pub username: &'b str,
    pub password: Option<&'b str>,
}

#[async_trait]
impl <'a> FromRequest<'a> for LoginRequest<'a> {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let username_opt = request.headers().get_one("username");
        let password_opt = request.headers().get_one("password");
        match username_opt {
            Some(username) => {
                let login_request = LoginRequest {username: username, password: password_opt};

                Outcome::Success(login_request)
            },
            None => Outcome::Failure((Status::BadRequest, "Username is required".to_owned()))
        }
    }
}

// I could not find a way to derive this because of the lifetime parameter
impl<'a> OpenApiFromRequest<'a> for LoginRequest<'a> {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}