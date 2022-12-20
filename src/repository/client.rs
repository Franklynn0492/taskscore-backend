use std::{env, collections::HashMap, sync::Arc};
use dotenv::dotenv;
use rocket::{tokio::{net::TcpStream, io::BufStream}, futures::lock::Mutex};

use bolt_client::{Client, bolt_proto::{version::{V4_3, V4_2}, Message, message::{Success, Record}, value::{Node, Relationship}}, Metadata, Params};
use tokio_util::compat::*;

#[cfg(test)]
use mockall::automock;

use crate::model::{Entity, Id, Relation};

use super::repository::DbActionError;

pub type ConnectionError = String;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait DbClient {
    async fn fetch<E: Entity> (&self, statement: String, params: Params) -> Result<Vec<E>, DbActionError>;
    async fn fetch_all<E: Entity> (&self) -> Result<Vec<E>, DbActionError>;
    async fn fetch_single<E: Entity> (&self, statement: String, params: Params) -> Result<Option<E>, DbActionError>;
    async fn fetch_by_id<E: Entity> (&self, id: &E::I) -> Result<Option<E>, DbActionError>;
    async fn create<E: Entity> (&self, statement: String, params: Params) -> Result<E, DbActionError>;
    async fn update<E: Entity> (&self, statement: String, params: Params) -> Result<E, DbActionError>;
    async fn delete<E: Entity> (&self, entity: &E) -> Result<(), DbActionError>;
    async fn create_relationship<S: Entity, T: Entity> (&self, source: Arc<S>, target: Arc<T>, name: &String, params_opt: Option<HashMap<&'static str, String>>) -> Result<Relation<S, T>, DbActionError>;
}

pub struct Neo4JClient {
    client: Mutex<Client<Compat<BufStream<TcpStream>>>>,
}

impl Neo4JClient {

    pub async fn connect() -> Result<Neo4JClient, ConnectionError> {
        dotenv().ok();
        let db_addr = env::var("TS_DATABASE_ADDRESS").or(Err("Database address not configured".to_owned()))?;

        info!("Connect to DB triggered; DB address: {}", db_addr);

        let stream = TcpStream::connect(db_addr).await.or(Err("unable to create TCP connection to database".to_owned()))?;
        let stream = BufStream::new(stream).compat();
    
        // Create a new connection to the server and perform a handshake to establish a
        // protocol version. This example demonstrates usage of the v4.3 or v4.2 protocol.
        debug!("Creating connection...");
        let result = Client::new(stream, &[V4_3, V4_2, 0, 0]).await;
        if (result.is_err()) {
            let error = format!("Connecting to database failed; error: {}", result.unwrap_err());
            error!("{}", error);
            return Err(error);
        }

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

    async fn pull_records(&self, client: &mut Client<Compat<BufStream<TcpStream>>>, metadata: Option<Metadata>) -> Result<Vec<Record>, DbActionError> {
        let pull_result = client.pull(metadata).await;
        if pull_result.is_err() {
            let com_err = pull_result.unwrap_err();
            let err_msg = format!("{}", com_err);
            println!("{}", err_msg);
            return Err(err_msg);
        }

        let (records, _response) = pull_result.unwrap();

        Ok(records)
    }

    async fn pull_entities<E: Entity>(&self, client: &mut Client<Compat<BufStream<TcpStream>>>, metadata: Option<Metadata>) -> Result<Vec<E>, DbActionError> {
        let records_result = self.pull_records(client, metadata).await;
        if (records_result.is_err()) {
            return Err(records_result.unwrap_err());
        }

        let records = records_result.unwrap();

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

    async fn pull_relations<S: Entity, T: Entity>(&self, client: &mut Client<Compat<BufStream<TcpStream>>>, metadata: Option<Metadata>, source_node: Arc<S>, target_node: Arc<T>) -> Result<Vec<Relation<S, T>>, DbActionError> {
        let records_result = self.pull_records(client, metadata).await;
        if (records_result.is_err()) {
            return Err(records_result.unwrap_err());
        }

        let records = records_result.unwrap();

        if records.len() == 0 {
            return Ok(vec![]);
        }

        let entities = records.into_iter().map(|record| {
            let relationship_result = Relationship::try_from(record.fields()[0].clone());

            if (relationship_result.is_ok()) {
                let relationship = relationship_result.unwrap();
                let relation_res = Relation::new(source_node.clone(), target_node.clone(), relationship.rel_type().to_string(), None);
                match relation_res {
                    Ok(r) => Ok(r),
                    Err(e) => Err(e)
                }
            } else {
                Err("Unable to create relation from record".to_owned())
            }
            
        }).collect::<Result<Vec<Relation<S, T>>,_>>();

        entities
    }

    async fn run(&self, statement: String, params_opt: Option<Params>) -> Result<(), DbActionError> {
        let mut client = self.client.lock().await;

        let run_result = client.run(statement, params_opt, None).await;
        
        if run_result.is_err() {
            let com_err = run_result.unwrap_err();
            let err_msg = format!("{}", com_err);
            println!("{}", err_msg);
            Err(err_msg)
        } else {
            Ok(())
        }
    }

    async fn perform_action_returning_one_entity<E: Entity>(&self, action_name: &str, statement: String, params_opt: Option<Params>) -> Result<E, DbActionError> {
        let run_result = self.run(statement, params_opt).await;
        
        if run_result.is_err() {
            return Err(run_result.unwrap_err());
        }

        let mut client = self.client.lock().await;
        let metadata = Some(Metadata::from_iter(vec![("n", 1)]));

        // this pull actually reads the new node we just created on the DB. It is not neccessary in order to complete the create
        let pull_result = self.pull_entities::<E>(&mut client, metadata).await;
        
        let result = pull_result.and_then(|mut entity_vec| entity_vec.pop().ok_or(format!("{} did not return entity", action_name)));

        //Neo4JRepository::discard(&mut client).await;

        result
    }

    async fn perform_action_returning_one_relation<S: Entity, T: Entity>(&self, action_name: &str, statement: String, params_opt: Option<Params>, source_node: Arc<S>, target_node: Arc<T>) -> Result<Relation<S, T>, DbActionError> {
        let run_result = self.run(statement, params_opt).await;
        
        if run_result.is_err() {
            return Err(run_result.unwrap_err());
        }

        let mut client = self.client.lock().await;
        let metadata = Some(Metadata::from_iter(vec![("n", 1)]));

        let pull_result = self.pull_relations::<S, T>(&mut client, metadata, source_node, target_node).await;
        
        let result = pull_result.and_then(|mut entity_vec| entity_vec.pop().ok_or(format!("{} did not return relation", action_name)));
        
        result
    }
}

#[async_trait]
impl DbClient for Neo4JClient {

    async fn fetch_by_id<E: Entity> (&self, id: &E::I) -> Result<Option<E>, DbActionError> {
        let statement = format!("MATCH (e: {}) WHERE id(e) = $id RETURN e", E::get_node_type_name());
        let params = Params::from_iter(vec![("id", id.to_string())]);

        self.fetch_single(statement, params).await
    }

    async fn fetch_all<E: Entity> (&self) -> Result<Vec<E>, DbActionError> {
        let statement = format!("MATCH (e: {}) RETURN e", E::get_node_type_name());
        let params = Params::from_iter::<Vec<(&str, &str)>>(vec![]);

        self.fetch(statement, params).await
    }

    async fn fetch<E: Entity> (&self, statement: String, params: Params) -> Result<Vec<E>, DbActionError> {
        let run_result = self.run(statement, Some(params)).await;
        
        if run_result.is_err() {
            return Err(run_result.unwrap_err());
        }

        let mut client = self.client.lock().await;

        let entities = self.pull_entities(&mut client, Some(Metadata::from_iter(vec![("n", i32::MAX)]))).await;
        //Neo4JClient::discard(&mut client).await;
        entities
    }

    async fn fetch_single<E: Entity> (&self, statement: String, params: Params) -> Result<Option<E>, DbActionError> {
        let fetch_result = self.fetch::<E>(statement, params).await;
        
        let result = fetch_result.and_then(|mut entity_vec| Ok(entity_vec.pop()));

        result
    }

    async fn create<E: Entity> (&self, statement: String, params: Params) -> Result<E, DbActionError> {
        
        let result = self.perform_action_returning_one_entity("Create", statement, Some(params)).await;

        result

    }

    async fn update<E: Entity> (&self, statement: String, params: Params) -> Result<E, DbActionError> {
        
        let result = self.perform_action_returning_one_entity("Update", statement, Some(params)).await;

        result
    }

    async fn delete<E: Entity> (&self, entity: &E) -> Result<(), DbActionError> {
        if entity.get_id().is_none() {
            return Err(format!("Entity {} is unpersisted and cannot be deleted", entity));
        }

        let statement = format!("MATCH (p:{}) WHERE id(p) = $id DETACH DELETE p", E::get_node_type_name());
        let params = Params::from_iter(vec![("id", entity.get_id().unwrap().to_string())]);

        let run_result = self.run(statement, Some(params)).await;
        
        if run_result.is_err() {
            return Err(run_result.unwrap_err());
        }

        Ok(())
    }

    async fn create_relationship<S: Entity, T: Entity> (&self, source: Arc<S>, target: Arc<T>, name: &String, params_opt: Option<HashMap<&'static str, String>>) -> Result<Relation<S, T>, DbActionError> {

        let relation_res = Relation::new(source.clone(), target.clone(), name.clone(), params_opt);
        if relation_res.is_err() {
            return Err(relation_res.unwrap_err());
        }

        let relation = relation_res.unwrap();
        let statement = relation.get_create_statement();

        let result = self.perform_action_returning_one_relation("Create relation", statement, None, source, target).await;

        result
    }
}