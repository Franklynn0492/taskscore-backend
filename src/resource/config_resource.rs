use dotenv::dotenv;
use rocket::{http::Status};
use rocket_okapi::openapi;
use std::env::{self};

use crate::model::{Session, http::KeyValueListResponder};

#[openapi]
#[get("/config")]
pub fn get_config<'a>(session: Session) -> KeyValueListResponder<String, String> {
    if !session.user.lock().unwrap().is_admin {
        return KeyValueListResponder::create(Status::Forbidden, vec![]);
    }

    dotenv().ok();
    let config_vec = env::vars().collect();

    KeyValueListResponder::create_ok(config_vec)
}