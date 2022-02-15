
use std::{env, iter::FromIterator, convert::TryFrom};

use bolt_client::{Client, bolt_proto::{version::{V4_3, V4_2}, Message, message::Success}, Metadata};
use dotenv::dotenv;
use rocket::{http::private::Connection, tokio::{net::TcpStream, io::BufStream}};
use tokio_util::compat::*;

pub struct Neo4JRepository {
    client: Client<Compat<BufStream<TcpStream>>>,
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

        Ok(Neo4JRepository { client })
    }
}