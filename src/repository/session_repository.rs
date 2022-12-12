use std::{sync::Arc};

use bolt_client::{Params};

use crate::{model::{Session, Entity}};

use super::{client::{Neo4JClient, DbClient, self}, repository::{ReadRepository, DbActionError, ModifyRepository, WriteRepository, ReadAllRepository}};

pub struct SessionRepository {
    client: Arc<Neo4JClient>,
}

impl SessionRepository {
    pub fn new(client: Arc<Neo4JClient>) -> SessionRepository {
        SessionRepository { client }
    }

    pub async fn find_session_by_session_id(&self, session_id: &String) -> Result<Option<Session>, DbActionError> {
        let statement = format!("MATCH (u:{} {{session_id: $session_id}}) RETURN u", Session::get_node_type_name());
        let params = Params::from_iter(vec![("session_id", session_id.clone())]);

        let result = self.client.fetch_single::<Session, u32>(statement, params).await;

        result
    }
}

#[async_trait]
impl  ModifyRepository<Session, u32> for SessionRepository {
    async fn update(&self, entity_with_update_values: &Session) -> Result<Session, DbActionError> {
        let statement = format!("MATCH (u:{}) WHERE id(u) = $id SET p.refreshed = $refreshed RETURN u", Session::get_node_type_name());
        let params = Params::from_iter(vec![("id", entity_with_update_values.id.to_string()), ("refreshed", entity_with_update_values.refreshed.to_string())]);
        
        let result = self.client.update::<Session, u32>(statement, params).await;

        result
    }
}

#[async_trait]
impl  WriteRepository<Session, u32> for SessionRepository {
    async fn add(&self, new_entity: &Session) -> Result<Session, DbActionError> {
        let statement = format!("CREATE (u:{} {{session_id: $session_id, started: $started, refreshed: &refreshed }}) RETURN u", Session::get_node_type_name());
        let params = Params::from_iter(vec![("session_id", new_entity.session_id.clone()),
            ("started", new_entity.started.to_string()), ("refreshed", new_entity.refreshed.to_string())]);
        
        let result = self.client.create::<Session, u32>(statement, params).await;

        if (result.is_ok()) {
            let session = result.unwrap();
            // ... i think I need a "Relationship"-Subtype of entity
            let 
        }

        result
    }

    async fn delete(&self, entity_to_delete: &Session) -> Result<(), DbActionError> {
        let result = self.client.delete::<Session, u32>(entity_to_delete).await;

        result
    }
}