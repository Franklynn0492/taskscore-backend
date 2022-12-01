use std::env;
use dotenv::dotenv;
use rocket::{tokio::{net::TcpStream, io::BufStream}, futures::lock::Mutex};

use bolt_client::{Client, bolt_proto::{version::{V4_3, V4_2}, Message, message::{Success, Record}, value::Node}, Metadata, Params};
use tokio_util::compat::*;

#[cfg(test)]
use mockall::automock;

type ConnectionError = String;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DbClient {
    async fn fetch(&self, statement: &str, params: Params) -> Option<Vec<Record>>;
    async fn create(&self, statement: &str) -> Result<Record, String>;
    async fn update(&self, statement: &str) -> Result<bool, String>;
    async fn delete(&self, statement: &str) -> Result<bool, String>;
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

    async fn match_in_db(client: &mut Client<Compat<BufStream<TcpStream>>>, statement: &str, params: Params) -> Option<Vec<Record>> {
        // Todo: remove; replaced by fetch
        !unimplemented!();
    }
}

#[async_trait]
impl DbClient for Neo4JClient {

    async fn fetch(&self, statement: &str, params: Params) -> Option<Vec<Record>> {
        let client = self.client.lock().await;

        let run_result = client.run(statement, Some(params), None).await;

        if run_result.is_err() {
            let err_msg = run_result.unwrap_err();
            println!("{}", err_msg);
            return None;
        }

        let metadata = Some(Metadata::from_iter(vec![("n", 1)]));

        let pull_result = client.pull(metadata).await;
        if pull_result.is_err() {
            let err_msg = pull_result.unwrap_err();
            println!("{}", err_msg);
            return None;
        }

        let (records, _response) = pull_result.unwrap();

        if records.len() == 0 {
            return None;
        }

        Neo4JClient::discard(client.unwrap()).await;

        Some(records)
        
    }

    async fn create(&self, statement: &str) -> Result<Record, String> {
        !unimplemented!();
    }

    async fn update(&self, statement: &str) -> Result<bool, String> {
        !unimplemented!();
    }

    async fn delete(&self, statement: &str) -> Result<bool, String> {
        !unimplemented!();
    }
}