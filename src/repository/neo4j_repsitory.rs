
use std::{env, iter::FromIterator, convert::TryFrom};

use bolt_client::{Client, bolt_proto::{version::{V4_3, V4_2}, Message, message::{Success, Record}, value::Node}, Metadata, Params};
use dotenv::dotenv;
use rocket::{tokio::{net::TcpStream, io::BufStream}, futures::lock::Mutex, http::Status};
use tokio_util::compat::*;
use crate::{model::{User}, resource::http::responder::MessageResponder};

use super::{legacy_repository::{LegacyRepository}, repository::Repository};

pub struct Neo4JRepository {
    client: Mutex<Client<Compat<BufStream<TcpStream>>>>,
    legacy_repo: LegacyRepository,  // TODO: Replace me
}

type ConnectionError = String;
impl Neo4JRepository {
    
}

#[async_trait]
impl Repository for Neo4JRepository {
    async fn get_user<'a>(&'a self, id: u32) -> Option<crate::model::User> {
        self.legacy_repo.get_user(id).await
    }

    // Todo: Remove (and rename find_user_by_username_const), is obsolete
    async fn find_user_by_username<'a>(&'a self, username: &String) -> Option<std::sync::Arc<std::sync::Mutex<crate::model::User>>> {
        self.legacy_repo.find_user_by_username(username).await
    }

    async fn find_user_by_username_const<'a>(&'a self, username: &String) -> Option<crate::model::User> {
        let mut client = self.client.lock().await;

        let statement = "MATCH (p:Person {username: $username}) RETURN p;";
        let params = Params::from_iter(vec![("username", username.clone())]);

        let records = Neo4JRepository::match_in_db(&mut client, statement, params).await?;

        let node = Node::try_from(records[0].fields()[0].clone()).unwrap();

        let user = User::from(node);

        return Some(user)
    }

    async fn get_all_users<'a>(&'a self) -> Vec<crate::model::User> {
        self.legacy_repo.get_all_users().await
    }

    async fn get_task<'a>(&'a self, id: u32) -> Option<crate::model::Task> {
        self.legacy_repo.get_task(id).await
    }

    async fn get_all_tasks<'a>(&'a self) -> Vec<crate::model::Task> {
        self.legacy_repo.get_all_tasks().await
    }

    async fn get_session<'a>(&'a self, session_id: &String) -> Option<crate::model::Session> {
        self.legacy_repo.get_session(session_id).await
    }

    async fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String> {
        self.legacy_repo.score(user_id, task_id).await
    }

    async fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String, is_admin: bool) -> Result<std::sync::Arc<std::sync::Mutex<crate::model::User>>, String> {
        self.legacy_repo.create_and_add_user(username, display_name, password, is_admin).await
    }

    async fn add_team<'a>(&'a self, team: crate::model::user::Team) -> Option<u32> {
        self.legacy_repo.add_team(team).await
    }

    async fn add_user_to_team<'a>(&'a self, team_name: &String, user_id: u32, manager: crate::model::User) -> Result<(), String> {
        self.legacy_repo.add_user_to_team(team_name, user_id, manager).await
    }

    async fn add_user<'a>(&'a self, _session: &crate::model::Session, user: crate::model::User) -> MessageResponder<u32> {
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

        Neo4JRepository::discard(&mut client).await;

        result
    }

    async fn login<'a>(&'a self, login_request: crate::model::session::LoginRequest) -> Result<crate::model::Session, String> {
        self.legacy_repo.login(login_request).await
    }

    async fn logout(&self, session_id: &String) -> Result<(), String> {
        self.legacy_repo.logout(session_id).await
    }
}