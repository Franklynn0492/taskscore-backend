
use std::{env, iter::FromIterator, convert::TryFrom, sync::{Arc, Mutex}};

use bolt_client::{Client, bolt_proto::{version::{V4_3, V4_2}, Message, message::Success, value::Node}, Metadata, Params};
use dotenv::dotenv;
use rocket::{tokio::{net::TcpStream, io::BufStream}, futures::executor::block_on};
use tokio_util::compat::*;

use crate::model::User;

use super::{legacy_repository::{LegacyRepository, self}, repository::Repository};

pub struct Neo4JRepository {
    client: Mutex<Client<Compat<BufStream<TcpStream>>>>,
    legacy_repo: LegacyRepository,  // TODO: Replace me
}

type ConnectionError = String;
impl Neo4JRepository {

    pub async fn connect() -> Result<Neo4JRepository, ConnectionError> {
        dotenv().ok();
        let db_addr = env::var("DATABASE_ADDRESS").or(Err("Database address not configured".to_owned()))?;

        let stream = TcpStream::connect(db_addr).await.or(Err("unable to create TCP connection to database".to_owned()))?;
        let stream = BufStream::new(stream).compat();
    
        // Create a new connection to the server and perform a handshake to establish a
        // protocol version. This example demonstrates usage of the v4.3 or v4.2 protocol.
        let mut result = Client::new(stream, &[V4_3, V4_2, 0, 0]).await;
        let mut client = result.unwrap();
         
        // Send a HELLO message with authentication details to the server to initialize
        // the session.
        let response: Message = client.hello(
            Metadata::from_iter(vec![
                ("user_agent", "my-client-name/1.0"),
                ("scheme", "basic"),
                ("principal", &env::var("DATABASE_PRINCIPAL").unwrap()),
                ("credentials", &env::var("DATABASE_PASSWORD").unwrap()),
            ])).await.or(Err("Error sending authentication info to database".to_owned()))?;

        Success::try_from(response).or(Err("DB responded with error on login".to_owned()))?;

        Ok(Neo4JRepository { client: Mutex::new(client), legacy_repo: LegacyRepository::init_repository() })
    }
}

impl Repository for Neo4JRepository {
    fn get_user<'a>(&'a self, id: u32) -> Option<crate::model::User> {
        todo!()
    }
    fn find_user_by_username<'a>(&'a self, username: &String) -> Option<std::sync::Arc<std::sync::Mutex<crate::model::User>>> {
        todo!()
    }

    fn find_user_by_username_const<'a>(&'a self, username: &String) -> Option<crate::model::User> {
        let statement = "MATCH (n:Person {{username: '$username'}}) Return n";
        let params = Params::from_iter(vec![("username", username.clone())]);
        let mut client = self.client.lock().unwrap();
        client.run(statement, Some(params), None);

        let (records, response) = block_on(client.pull(Some(Metadata::from_iter(vec![("n", "1")])))).unwrap();

        if records.len() == 0 {
            return None;
        }

        let node = Node::try_from(records[0].fields()[0].clone()).unwrap();

        let user = User::from(node);

        return Some(user)
    }

    fn get_all_users<'a>(&'a self) -> Vec<crate::model::User> {
        self.legacy_repo.get_all_users()
    }

    fn get_task<'a>(&'a self, id: u32) -> Option<crate::model::Task> {
        todo!()
    }

    fn get_all_tasks<'a>(&'a self) -> Vec<crate::model::Task> {
        todo!()
    }

    fn get_session<'a>(&'a self, session_id: &String) -> Option<crate::model::Session> {
        todo!()
    }

    fn score<'a>(&'a self, user_id: u32, task_id: u32) -> Result<u16, String> {
        todo!()
    }

    fn create_and_add_user<'a>(&'a self, username: String, display_name: String, password: String, is_admin: bool) -> Result<std::sync::Arc<std::sync::Mutex<crate::model::User>>, String> {
        todo!()
    }

    fn add_team<'a>(&'a self, team: crate::model::user::Team) -> Option<u32> {
        todo!()
    }

    fn add_user_to_team<'a>(&'a self, team_name: &String, user_id: u32, manager: crate::model::User) -> Result<(), String> {
        todo!()
    }

    fn add_user<'a>(&'a self, session: &crate::model::Session, user: crate::model::User) -> crate::model::MessageResponder<u32> {
        todo!()
    }

    fn login(&self, login_request: crate::model::session::LoginRequest) -> Result<crate::model::Session, String> {
        todo!()
    }

    fn logout(&self, session_id: &String) -> Result<(), String> {
        todo!()
    }
}