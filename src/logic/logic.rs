use std::{sync::{Arc, Mutex}, rc::Rc};
use crate::repository::client::{Neo4JClient, DbClient};

#[cfg(test)]
use mockall::automock;

use crate::{model::{User, Task, Session, session::LoginRequest, user::Team}, resource::http::responder::MessageResponder, repository::{user_repository::UserRepository, neo4j_repsitory::Neo4JRepository}};

#[cfg_attr(test, automock)] // Apply the automock macro only if you are testing
#[async_trait]
pub trait Logic {

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
    async fn login<'a>(&'a self, login_request: LoginRequest) -> Result<Session, String>;
    async fn logout(&self, session_id: &String) -> Result<(), String>;
}

// This struct will become quite big (or at least its iplementation) Might have to break it up sooner or later.
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

