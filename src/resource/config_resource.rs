use dotenv::dotenv;
use rocket::{serde::json::Json, http::Status};
use std::env::{self};

use crate::model::Session;

#[get("/config")]
pub fn get_config<'a>(session: Session) -> (Status, Json<Vec<(String, String)>>) {
    if !session.user.lock().unwrap().is_admin {
        return (Status::Forbidden, Json(vec![]));
    }

    dotenv().ok();
    let config_vec = env::vars().collect();

    (Status::Ok, Json(config_vec))
}