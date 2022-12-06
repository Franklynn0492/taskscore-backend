use std::env;
use dotenv::dotenv;
use rocket::{tokio::{net::TcpStream, io::BufStream}, futures::lock::Mutex};

use bolt_client::{Client, bolt_proto::{version::{V4_3, V4_2}, Message, message::{Success, Record}, value::Node}, Metadata, Params};
use tokio_util::compat::*;

#[cfg(test)]
use mockall::automock;

use crate::model::Entity;

use super::repository::DbActionError;

pub type ConnectionError = String;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DbClient {
    async fn fetch<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<Vec<E>, DbActionError>;
    async fn fetch_single<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<Option<E>, DbActionError>;
    async fn create<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<E, DbActionError>;
    async fn update<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<E, DbActionError>;
    async fn delete<E: Entity<I>, I: Send + Sync + 'static> (&self, entity: E) -> Result<bool, DbActionError>;
}

pub struct Neo4JClient {
    client: Mutex<Client<Compat<BufStream<TcpStream>>>>,
}

impl Neo4JClient {

    pub async fn connect() -> Result<Neo4JClient, ConnectionError> {
        dotenv().ok();
        let db_addr = env::var("TS_DATABASE_ADDRESS").or(Err("Database address not configured".to_owned()))?;

        let stream = TcpStream::connect(db_addr).await.or(Err("unable to create TCP connection to database".to_owned()))?;
        let stream = BufStream::new(stream).compat();
    
        // Create a new connection to the server and perform a handshake to establish a
        // protocol version. This example demonstrates usage of the v4.3 or v4.2 protocol.
        let result = Client::new(stream, &[V4_3, V4_2, 0, 0]).await;
        let mut client = result.unwrap();
         
        // Send a HELLO message with authentication details to the server to initialize
        // the session.
        let response: Message = client.hello(
            Metadata::from_iter(vec![
                ("user_agent", "my-client-name/1.0"),
                ("scheme", "basic"),
                ("principal", &env::var("TS_DATABASE_PRINCIPAL").unwrap()),
                ("credentials", &env::var("TS_DATABASE_PASSWORD").unwrap()),
            ])).await.or(Err("Error sending authentication info to database".to_owned()))?;

        Success::try_from(response).or(Err("DB responded with error on login".to_owned()))?;

        Ok(Neo4JClient { client: Mutex::new(client) })
    }

    async fn discard(client: &mut Client<Compat<BufStream<TcpStream>>>) {
        let discard_result = client.discard(Some(Metadata::from_iter(vec![("n", -1)]))).await;

        if discard_result.is_err() {
            let err_msg = discard_result.unwrap_err();
            println!("{}", err_msg);
        }
    }

    async fn pull<E: Entity<I>, I: Send + Sync + 'static>(&self, client: &mut Client<Compat<BufStream<TcpStream>>>, metadata: Option<Metadata>) -> Result<Vec<E>, DbActionError> {

        let pull_result = client.pull(metadata).await;
        if pull_result.is_err() {
            let com_err = pull_result.unwrap_err();
            let err_msg = format!("{}", com_err);
            println!("{}", err_msg);
            return Err(err_msg);
        }

        let (records, _response) = pull_result.unwrap();

        if records.len() == 0 {
            return Ok(vec![]);
        }

        let entities = records.into_iter().map(|record| {
            let node_result = Node::try_from(record.fields()[0].clone());

            if (node_result.is_ok()) {
                Ok(E::from(node_result.unwrap()))
            } else {
                Err("Unable to create node from record".to_owned())
            }
            
        }).collect::<Result<Vec<E>,_>>(); // Collecting into a result, in case a map fails. See: https://www.reddit.com/r/rust/comments/omsukl/falliable_iterators_why_no_try_map_for_iterator/

        entities
    }
}

#[async_trait]
impl DbClient for Neo4JClient {

    async fn fetch<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<Vec<E>, DbActionError> { // TODO: Check if this can be improved
        let client = self.client.lock().await;

        let run_result = client.run(statement, Some(params), None).await;

        if run_result.is_err() {
            let com_err = run_result.unwrap_err();
            let err_msg = format!("{}", com_err);
            println!("{}", err_msg);
            return Err(err_msg);
        }

        let entities = self.pull(&mut client, Some(Metadata::from_iter(vec![("n", 1)]))).await;
        Neo4JClient::discard(&mut client).await;
        entities
    }

    async fn fetch_single<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<Option<E>, DbActionError> {
        let fetch_result = self.fetch::<E, I>(statement, params).await;
        
        let result = fetch_result.and_then(|entity_vec| Ok(entity_vec.pop()));

        result
    }

    async fn create<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<E, DbActionError> {
        let client = self.client.lock().await;

        let run_result = client.run(statement, Some(params), None).await;
        
        if run_result.is_err() {
            let com_err = run_result.unwrap_err();
            let err_msg = format!("{}", com_err);
            println!("{}", err_msg);
            return Err(err_msg);
        }

        let metadata = Some(Metadata::from_iter(vec![("n", 1)]));

        

        let pull_result = self.pull::<E, I>(&mut client, metadata).await;
        
        let result = pull_result.and_then(|entity_vec| entity_vec.pop().ok_or("Create did not return entity".to_owned()));

        //Neo4JRepository::discard(&mut client).await;

        result

    }

    async fn update<E: Entity<I>, I: Send + Sync + 'static> (&self, statement: &str, params: Params) -> Result<E, DbActionError> {
        !unimplemented!();
    }

    async fn delete<E: Entity<I>, I: Send + Sync + 'static> (&self, entity: E) -> Result<bool, DbActionError> {
        !unimplemented!();
    }
}