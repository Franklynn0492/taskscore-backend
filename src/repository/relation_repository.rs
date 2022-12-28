use std::{sync::{Arc, Mutex}, collections::HashMap};

use bolt_client::{bolt_proto::Value};

use crate::{model::{entity::Relation}};
use crate::model::entity::{Entity};

use super::{client::{Neo4JClient, DbClient}, repository::{DbActionError}};

pub struct RelationRepository {
    client: Arc<Neo4JClient>,
}

impl RelationRepository {
    pub fn new(client: Arc<Neo4JClient>) -> RelationRepository {
        RelationRepository { client }
    }


    pub async fn create_relationship<S: Entity, T: Entity> (&self, source: Arc<Mutex<S>>, target: Arc<Mutex<T>>, name: &String, params_opt: Option<HashMap<String, Value>>) -> Result<Relation<S, T>, DbActionError> {
        let result = self.client.create_relationship(source.clone(), target.clone(), name, params_opt).await;
        result
    }

    pub async fn fetch_relations_of_node_of_type<S: Entity, T: Entity>(&self, source: Arc<Mutex<S>>, name: &String) -> Result<Vec<Relation<S, T>>, DbActionError> {
        let result = self.client.fetch_relations_of_node_of_type(source.clone(), name).await;
        result
    }

    pub async fn fetch_relations_of_type<S: Entity, T: Entity>(&self, name: &String) -> Result<Vec<Relation<S, T>>, DbActionError> {
        let result = self.client.fetch_relations_of_type(name).await;
        result
    }

    pub async fn fetch_single_relation<S: Entity, T: Entity>(&self, source: Arc<Mutex<S>>, target: Arc<Mutex<T>>, name: &String) -> Result<Relation<S, T>, DbActionError> {
        let result = self.client.fetch_single_relation(source, target, name).await;
        result
    }

    pub async fn delete_relation<S: Entity, T: Entity>(&self, source: &S, target: &T, name: &String) -> Result<(), DbActionError> {
        let result = self.client.delete_relation(source, target, name).await;
        result
    }
}