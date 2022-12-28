use std::{sync::{Arc, Mutex}};

use bolt_client::{Params, bolt_proto::Value};

use crate::model::{entity::{Entity, Relation}, user::Team, User};

use super::{client::{Neo4JClient, DbClient}, repository::{ReadRepository, DbActionError, ModifyRepository, WriteRepository, ReadAllRepository}};

pub struct TeamRepository {
    client: Arc<Neo4JClient>,
}

impl  TeamRepository {
    pub fn new(client: Arc<Neo4JClient>) -> TeamRepository {
        TeamRepository { client }
    }

    pub async fn fill_team_members(&self, team: Arc<Mutex<Team>>) -> Result<(), DbActionError> {
        let fetch_result = self.client.fetch_relations_of_node_of_type(team.clone(), &"MEMBER".to_owned()).await;

        if (fetch_result.is_err()) {
            return Err(fetch_result.unwrap_err());
        }

        let members_relations: Vec<Relation<Team, User>> = fetch_result.unwrap();
        let members: Vec<Arc<Mutex<User>>> = members_relations.into_iter().map(|relation| relation.target_node).collect();
        team.lock().unwrap().members = members;

        Ok(())
    }
}

#[async_trait]
impl  ReadRepository<Team> for TeamRepository {
    async fn find_by_id(&self, id: &u32) -> Result<Option<Team>, DbActionError> {
        let result = self.client.fetch_by_id::<Team>(&id).await;

        result
    }
}

#[async_trait]
impl  ModifyRepository<Team> for TeamRepository {
    async fn update(&self, entity_with_update_values: &Team) -> Result<Team, DbActionError> {
        if entity_with_update_values.get_id().is_none() {
            return Err(format!("Id of {} entity {} is unknown; entity cannot be modified", Team::get_node_type_name(), entity_with_update_values));
        }

        let statement = format!("MATCH (u:{}) WHERE id(u) = $id SET u.name = $name RETURN u", Team::get_node_type_name());
        let params = Params::from_iter(vec![("id", Value::Integer(entity_with_update_values.get_id().as_ref().unwrap().clone().into())), ("name", Value::String(entity_with_update_values.name.clone()))]);
        
        let result = self.client.update::<Team>(statement, params).await;

        result
    }
}

#[async_trait]
impl  WriteRepository<Team> for TeamRepository {
    async fn add(&self, new_entity: &Team) -> Result<Arc<Team>, DbActionError> {
        let statement = format!("CREATE (u:{} {{name: $name }}) RETURN u", Team::get_node_type_name());
        let params = Params::from_iter(vec![("name", Value::String(new_entity.name.clone()))]);
        
        let result = self.client.create::<Team>(statement, params).await;

        if result.is_ok() {
            let team = result.unwrap();
            let manager = team.manager.clone();

            let relationship_result = self.client.create_relationship(Arc::new(Mutex::new(team.clone())), manager, &"MANAGED_BY".to_owned(), None).await;

            relationship_result.map(|_| Arc::new(team))
        } else {
            Err(result.unwrap_err())
        }

    }

    async fn delete(&self, entity_to_delete: &Team) -> Result<(), DbActionError> {
        let result = self.client.delete::<Team>(entity_to_delete).await;

        result
    }
}

#[async_trait]
impl  ReadAllRepository<Team> for TeamRepository {
    async fn find_all(&self) -> Result<Vec<Team>, DbActionError> {
        let result = self.client.fetch_all::<Team>().await;

        result
    }
}

