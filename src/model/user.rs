
use std::{sync::{Arc, Mutex}, hash::Hash, collections::HashSet};

use bcrypt::{DEFAULT_COST};
use rocket::{Request, request::Outcome, http::Status, request::{ FromRequest}};
use rocket_okapi::OpenApiFromRequest;
use schemars::JsonSchema;

use crate::repository::repository::Repository;

use super::{Task, Score};

#[derive(serde::Serialize, Clone, JsonSchema, OpenApiFromRequest)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub display_name: String,
    pub is_admin: bool,
    pub points: u16,
    
    #[serde(skip_serializing)]
    pub scores: Vec<Score>,
    
    #[serde(skip_serializing)]
    pwd_hash_components: Option<String>,
}

impl User {
    pub fn new(id: u32, username: String, display_name: String, is_admin: bool) -> User {
        User {id, username: username, display_name: display_name, points: 0, scores: vec![], pwd_hash_components: None, is_admin}
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
                let mut new_user = User::new(0, username.to_owned(), display_name_opt.unwrap_or(username).to_owned(), false);
                if password_opt.is_some() {
                    new_user.set_password(password_opt.unwrap().to_owned());
                }

                Outcome::Success(new_user)
            },
            None => Outcome::Failure((Status::BadRequest, "Username is required".to_owned()))
        }
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {
    //fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for User {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.username.hash(state);
        self.display_name.hash(state);
        self.is_admin.hash(state);
    }
}

#[derive(Clone)]
#[derive(serde::Serialize)]
pub struct Team {
    pub id: u32,
    pub name: String,
    pub manager_id: u32,
    pub members: Vec<Arc<Mutex<User>>>,
    pub member_ids: HashSet::<u32>,
}

impl Team {
    pub fn new(id: u32, name: String, manager: Arc<Mutex<User>>) -> Team {
        let members = vec![manager.clone()];
        let mut member_ids = HashSet::new();
        member_ids.insert(manager.lock().unwrap().id);
        Team { id, name, manager_id: manager.lock().unwrap().id, members, member_ids }
    }

    pub fn add_user(&mut self, new_user: Arc<Mutex<User>>, authority: &User) -> Result<(), String> {
        let new_user_locked = new_user.lock().unwrap();
        if self.contains(&new_user_locked) {
            return Err(format!("User '{}' is already member of group '{}'", new_user_locked.username, self.name));
        }

        if self.manager_id != authority.id && !authority.is_admin {
            return Err(format!("User '{}' is not authorized to add users to group '{}'", authority.username, self.name));
        }

        self.member_ids.insert(new_user_locked.id);
        self.members.push(new_user.clone());

        Ok(())
    }

    pub fn contains(&self, user: &User) -> bool {
        self.member_ids.contains(&user.id)
    }
}

#[async_trait]
impl <'a> FromRequest<'a> for Team {
    type Error = String;

    async fn from_request(request: &'a Request<'_>) -> Outcome<Self, Self::Error> {
        let teamname_opt = request.headers().get_one("teamname");//.ok_or("Team name is required")?;
        let user_id_opt = request.headers().get_one("userid");
        let state = request.rocket().state::<Repository>().unwrap();

        if teamname_opt.is_none() {
            return Outcome::Failure((Status::BadRequest, "Team name is required".to_owned()));
        }

        if user_id_opt.is_none() {
            return Outcome::Failure((Status::BadRequest, "UserId is required".to_owned()));
        }

        let user_id = user_id_opt.unwrap();
        let user_id_parsed = str::parse::<u32>(user_id_opt.unwrap());
        if user_id_parsed.is_err() {
            return Outcome::Failure((Status::BadRequest, format!("'{}' is not a valid user id", user_id)));
        }

        let user_id = user_id_parsed.unwrap();
        let manager_opt = state.get_user(user_id);

        if manager_opt.is_none() {
            return Outcome::Failure((Status::NotFound, format!("UserId '{}' is unknown", user_id)));
        }

        Outcome::Success(Team::new(0, teamname_opt.unwrap().to_owned(), Arc::new(Mutex::new(manager_opt.unwrap()))))
    }
}
