use std::{sync::Arc};

use bolt_client::{Params, bolt_proto::Value};

use crate::{model::{Task}};
use crate::model::entity::{Entity};

use super::{client::{Neo4JClient, DbClient}, repository::{ReadRepository, DbActionError, ModifyRepository, WriteRepository, ReadAllRepository}};

pub struct TaskRepository {
    client: Arc<Neo4JClient>,
}

impl  TaskRepository {
    pub fn new(client: Arc<Neo4JClient>) -> TaskRepository {
        TaskRepository { client }
    }
}

#[async_trait]
impl  ReadRepository<Task> for TaskRepository {
    async fn find_by_id(&self, id: &u32) -> Result<Option<crate::model::Task>, DbActionError> {
        let result = self.client.fetch_by_id::<Task>(&id).await;

        result
    }
}

#[async_trait]
impl  ModifyRepository<Task> for TaskRepository {
    async fn update(&self, entity_with_update_values: &Task) -> Result<Task, DbActionError> {
        if entity_with_update_values.get_id().is_none() {
            return Err(format!("Id of entity {} is unknown; entity cannot be modified", entity_with_update_values));
        }

        let statement = format!("MATCH (u:{}) WHERE id(u) = $id SET u.name = $name, u.points = $points, u.enabled = $enabled RETURN u", Task::get_node_type_name());
        let params = Params::from_iter(vec![("id", Value::Integer(entity_with_update_values.get_id().as_ref().unwrap().clone().into())), ("name", Value::String(entity_with_update_values.name.clone())),
            ("points", Value::Integer(entity_with_update_values.points.into())), ("enabled", Value::Boolean(entity_with_update_values.enabled))]);
        
        let result = self.client.update::<Task>(statement, params).await;

        result
    }
}

#[async_trait]
impl  WriteRepository<Task> for TaskRepository {
    async fn add(&self, new_entity: &Task) -> Result<Arc<Task>, DbActionError> {
        let statement = format!("CREATE (u:{} {{name: $name, points: $points, enabled: $enabled }}) RETURN u", Task::get_node_type_name());
        let params = Params::from_iter(vec![("name", Value::String(new_entity.name.clone())), ("points", Value::Integer(new_entity.points.into())),
            ("enabled", Value::Boolean(new_entity.enabled))]);
        
        let result = self.client.create::<Task>(statement, params).await;

        result.map(|u| Arc::new(u))
    }

    async fn delete(&self, entity_to_delete: &Task) -> Result<(), DbActionError> {
        let result = self.client.delete::<Task>(entity_to_delete).await;

        result
    }
}

#[async_trait]
impl  ReadAllRepository<Task> for TaskRepository {
    async fn find_all(&self) -> Result<Vec<Task>, DbActionError> {
        let result = self.client.fetch_all::<Task>().await;

        result
    }
}

