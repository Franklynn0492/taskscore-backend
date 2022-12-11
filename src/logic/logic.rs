use std::{sync::{Arc, Mutex}};
use crate::repository::client::{Neo4JClient};
use crate::repository::repository::*;

#[cfg(test)]
use mockall::automock;

use crate::{model::{User, Task, Session, session::LoginRequest, user::Team}, resource::http::responder::MessageResponder, repository::{user_repository::UserRepository}};

#[cfg_attr(test, automock)] // Apply the automock macro only if you are testing
#[async_trait]
pub trait Logic {

    async fn get_user(&self, id: u32) -> Option<User>;
    async fn find_user_by_username(&self, username: &String) -> Option<User>;
    async fn get_all_users(&self) -> Vec<User>;
    async fn get_task(&self, id: u32) -> Option<Task>;
    async fn get_all_tasks(&self) -> Vec<Task>;
    async fn get_session(&self, session_id: &String) -> Option<Session>;
    async fn score(&self, user_id: u32, task_id: u32) -> Result<u16, String>;
    async fn create_and_add_user(&self, username: String, display_name: String, password: String, is_admin: bool) -> Result<Arc<Mutex<User>>, String>;
    async fn add_team(&self, team: Team) -> Option<u32>;
    async fn add_user_to_team(&self, team_name: &String, user_id: u32, manager: User) -> Result<(), String>;
    async fn add_user(&self, session: &Session, user: User) -> MessageResponder<u32>;
    async fn login(&self, login_request: LoginRequest) -> Result<Session, String>;
    async fn logout(&self, session_id: &String) -> Result<(), String>;
}

// This struct will become quite big (or at least its implementation) Might have to break it up sooner or later.
pub struct ApplicationLogic {
    db_client: Arc<Neo4JClient>,
    user_repo: UserRepository,
}
pub type ApplicationLogicError = String;

impl ApplicationLogic {
    pub async fn new() -> Result<ApplicationLogic, ApplicationLogicError> {
        let db_client = Arc::new(Neo4JClient::connect().await?);

        let user_repo = UserRepository::new(db_client.clone());

        Ok(ApplicationLogic { db_client, user_repo })
    }
    
}

#[async_trait]
impl Logic for ApplicationLogic {


    async fn get_user(&self, id: u32) -> Option<User> {
        let user_res = self.user_repo.find_by_id(&id).await;
        user_res.unwrap_or_else(|msg| {error!("{}", msg); None})
    }

    async fn find_user_by_username(&self, username: &String) -> Option<User> {
        let user_res = self.user_repo.find_user_by_username(&username).await;
        user_res.unwrap_or_else(|msg| {error!("{}", msg); None})
    }
    
    async fn get_all_users(&self) -> Vec<User> {
        let user_res = self.user_repo.find_all().await;
        user_res.unwrap_or_else(|msg| {error!("{}", msg); vec![]})  // TODO: Unsure about swallowing the error message
    }
    
    async fn get_task(&self, id: u32) -> Option<Task> {
        !unimplemented!();
    }
    
    async fn get_all_tasks(&self) -> Vec<Task> {
        !unimplemented!();
    }
    
    async fn get_session(&self, session_id: &String) -> Option<Session> {
        !unimplemented!();
    }
    
    async fn score(&self, user_id: u32, task_id: u32) -> Result<u16, String> {
        !unimplemented!();
    }
    
    async fn create_and_add_user(&self, username: String, display_name: String, password: String, is_admin: bool) -> Result<Arc<Mutex<User>>, String> {
        !unimplemented!();
    }
    
    async fn add_team(&self, team: Team) -> Option<u32> {
        !unimplemented!();
    }
    
    async fn add_user_to_team(&self, team_name: &String, user_id: u32, manager: User) -> Result<(), String> {
        !unimplemented!();
    }
    
    async fn add_user(&self, session: &Session, user: User) -> MessageResponder<u32> {
        !unimplemented!();
    }
    
    async fn login(&self, login_request: LoginRequest) -> Result<Session, String> {
        !unimplemented!();
    }
    
    async fn logout(&self, session_id: &String) -> Result<(), String> {
        !unimplemented!();
    }
    
}