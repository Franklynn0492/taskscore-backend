
use std::{env, iter::FromIterator, convert::TryFrom};

use bolt_client::{Client, bolt_proto::{version::{V4_3, V4_2}, Message, message::{Success, Record}, value::Node}, Metadata, Params};
use dotenv::dotenv;
use rocket::{tokio::{net::TcpStream, io::BufStream}, futures::lock::Mutex, http::Status};
use tokio_util::compat::*;
use crate::{model::{User}, resource::http::responder::MessageResponder};

pub struct Neo4JRepository {
    client: Mutex<Client<Compat<BufStream<TcpStream>>>>,
}

type ConnectionError = String;
impl Neo4JRepository {
    
}

impl Neo4JRepository {
    pub async fn get_user<'a>(&'a self, id: u32) -> Option<crate::model::User> {
        //self.legacy_repo.get_user(id).await
        !unimplemented!();
    }

    // Todo: Remove (and rename find_user_by_username_const), is obsolete
    pub async fn find_user_by_username<'a>(&'a self, username: &String) -> Option<std::sync::Arc<std::sync::Mutex<crate::model::User>>> {
        //self.legacy_repo.find_user_by_username(username).await
        !unimplemented!();
    }

    pub async fn get_all_users<'a>(&'a self) -> Vec<crate::model::User> {
        //self.legacy_repo.get_all_users().await
        !unimplemented!();
    }

    pub async fn get_task<'a>(&'a self, id: u32) -> Option<crate::model::Task> {
        //self.legacy_repo.get_task(id).await
        !unimplemented!();
    }

    pub async fn get_all_tasks<'a>(&'a self) -> Vec<crate::model::Task> {
        //self.legacy_repo.get_all_tasks().await
        !unimplemented!();
    }

    pub async fn get_session<'a>(&'a self, session_id: &String) -> Option<crate::model::Session> {
        //self.legacy_repo.get_session(session_id).await
        !unimplemented!();
    }

    pub async fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String> {
        //self.legacy_repo.score(user_id, task_id).await
        !unimplemented!();
    }

    pub async fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String, is_admin: bool) -> Result<std::sync::Arc<std::sync::Mutex<crate::model::User>>, String> {
        //self.legacy_repo.create_and_add_user(username, display_name, password, is_admin).await
        !unimplemented!();
    }

    pub async fn add_team<'a>(&'a self, team: crate::model::user::Team) -> Option<u32> {
        //self.legacy_repo.add_team(team).await
        !unimplemented!();
    }

    pub async fn add_user_to_team<'a>(&'a self, team_name: &String, user_id: u32, manager: crate::model::User) -> Result<(), String> {
        //self.legacy_repo.add_user_to_team(team_name, user_id, manager).await
        !unimplemented!();
    }

    pub async fn add_user<'a>(&'a self, _session: &crate::model::Session, user: crate::model::User) -> MessageResponder<u32> {
        // Todo: Check if admin session
        // It is needed tochange the Mutex within it to the rocket version - and this is toom much work for me right now :)

        let mut client = self.client.lock().await;

        let statement = "CREATE (:Person {username: $username, display_name: $display_name, password: $pwd_hash_components, is_admin: $is_admin });";
        let params = Params::from_iter(vec![
            ("username", user.username),
            ("display_name", user.display_name),
            ("pwd_hash_components", user.pwd_hash_components.unwrap_or("".to_owned())),
            ("is_admin", format!("{}", user.is_admin))]);

        let run_result = client.run(statement, Some(params), None).await;
        
        let result;
        if run_result.is_err() {
            let err_msg = run_result.unwrap_err();
            println!("{}", err_msg);
            result = MessageResponder::create_with_message(Status::InternalServerError, "Error running create on db (run)".to_owned());
        } else {

            let metadata = Some(Metadata::from_iter(vec![("n", 1)]));
            let pull_result = client.pull(metadata).await;
            if pull_result.is_err() {
                let err_msg = run_result.unwrap_err();
                println!("{}", err_msg);
                result = MessageResponder::create_with_message(Status::InternalServerError, "Error running create on db (run)".to_owned());
            } else {
                result = MessageResponder::create_ok(0);
            }
        }

        //Neo4JRepository::discard(&mut client).await;

        result
    }

    pub async fn login<'a>(&'a self, login_request: crate::model::session::LoginRequest) -> Result<crate::model::Session, String> {
        //self.legacy_repo.login(login_request).await
        !unimplemented!();
    }

    pub async fn logout(&self, session_id: &String) -> Result<(), String> {
        //self.legacy_repo.logout(session_id).await
        !unimplemented!();
    }
}