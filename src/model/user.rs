
use std::convert::TryFrom;

use bcrypt::{DEFAULT_COST, hash, verify};
use rocket::{Request, request::Outcome, http::Status, request::{ FromRequest}};

use super::{Task, Score};

#[derive(serde::Serialize)]
#[derive(Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub display_name: String,
    pub points: u16,
    
    #[serde(skip_serializing)]
    pub scores: Vec<Score>,
    
    #[serde(skip_serializing)]
    pwd_hash_components: Option<String>,
}

impl User {
    pub fn new(id: u32, username: String, display_name: String) -> User {
        User {id, username: username, display_name: display_name, points: 0, scores: vec![], pwd_hash_components: None}
    }

    pub fn score_task<'a>(& mut self, task: Task) {
        self.points += task.points;

        let score = Score::new(task);
        self.scores.push(score);
    }

    pub fn set_password(&mut self, password: String) {
        self.pwd_hash_components = Some(bcrypt::hash(password, DEFAULT_COST).unwrap());
    }

    pub fn verify_password(&self, password_to_verify: Option<&str>) -> bool {
        match &self.pwd_hash_components {
            Some(hash) => match password_to_verify {
                Some(pwd_to_verify) => bcrypt::verify(pwd_to_verify, hash.as_str()).unwrap(),
                None => false,
            }
            None => true
        }
    }
}

#[async_trait]
impl <'a> FromRequest<'a> for User {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let username_opt = request.headers().get_one("username");
        let display_name_opt = request.headers().get_one("display_name");
        let password_opt = request.headers().get_one("password");
        match username_opt {
            Some(username) => {
                let mut new_user = User::new(0, username.to_owned(), display_name_opt.unwrap_or(username).to_owned());
                if password_opt.is_some() {
                    new_user.set_password(password_opt.unwrap().to_owned());
                }

                Outcome::Success(new_user)
            },
            None => Outcome::Failure((Status::BadRequest, "Username is required".to_owned()))
        }
    }
}
