use std::sync::{Arc, Mutex};

use chrono::{DateTime, Utc};
use rocket::{Request, request::Outcome, http::Status, request::FromRequest};

use super::User;
use rand::Rng;

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
const PASSWORD_LEN: usize = 30;

#[derive(serde::Serialize)]
#[derive(Clone)]
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
}

#[derive(serde::Serialize)]
#[derive(Clone)]
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