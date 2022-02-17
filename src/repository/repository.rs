use std::sync::{Arc, Mutex};

use crate::model::{User, Task, Session, MessageResponder, session::LoginRequest, user::Team};

#[async_trait]
pub trait Repository {

    async fn get_user<'a>(&'a self, id: u32) -> Option<User>;
    async fn find_user_by_username<'a>(&'a self, username: &String) -> Option<Arc<Mutex<User>>>;
    async fn find_user_by_username_const<'a>(&'a self, username: &String) -> Option<User>;
    async fn get_all_users<'a>(&'a self) -> Vec<User>;
    async fn get_task<'a>(&'a self, id: u32) -> Option<Task>;
    async fn get_all_tasks<'a>(&'a self) -> Vec<Task>;
    async fn get_session<'a>(&'a self, session_id: &String) -> Option<Session>;
    async fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String>;
    async fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String, is_admin: bool) -> Result<Arc<Mutex<User>>, String>;
    async fn add_team<'a>(&'a self, team: Team) -> Option<u32>;
    async fn add_user_to_team<'a>(&'a self, team_name: &String, user_id: u32, manager: User) -> Result<(), String>;
    async fn add_user<'a>(&'a self, session: &Session, user: User) -> MessageResponder<u32>;
    async fn login<'a>(&'a self, login_request: LoginRequest<'a>) -> Result<Session, String>;
    async fn logout(&self, session_id: &String) -> Result<(), String>;
}

pub trait SizedRepository: Repository + Sized {}
