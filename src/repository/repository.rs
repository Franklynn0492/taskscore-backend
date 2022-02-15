use std::sync::{Arc, Mutex};

use crate::model::{User, Task, Session, MessageResponder, session::LoginRequest, user::Team};

pub trait Repository {

    fn get_user<'a>(&'a self, id: u32) -> Option<User>;
    fn find_user_by_username<'a>(&'a self, username: &String) -> Option<Arc<Mutex<User>>>;
    fn get_all_users<'a>(&'a self) -> Vec<User>;
    fn get_task<'a>(&'a self, id: u32) -> Option<Task>;
    fn get_all_tasks<'a>(&'a self) -> Vec<Task>;
    fn get_session<'a>(&'a self, session_id: &String) -> Option<Session>;
    fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String>;
    fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String, is_admin: bool) -> Result<Arc<Mutex<User>>, String>;
    fn add_team<'a>(&'a self, team: Team) -> Option<u32>;
    fn add_user_to_team<'a>(&'a self, team_name: &String, user_id: u32, manager: User) -> Result<(), String>;
    fn add_user<'a>(&'a self, session: &Session, user: User) -> MessageResponder<u32>;
    fn login(&self, login_request: LoginRequest) -> Result<Session, String>;
    fn logout(&self, session_id: &String) -> Result<(), String>;
}

pub trait SizedRepository: Repository + Sized {}
