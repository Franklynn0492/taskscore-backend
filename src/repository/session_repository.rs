use std::{sync::{Arc, Mutex}};

use bolt_client::{Params};

use crate::{model::{Session, Entity, User}};

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

        let result = self.client.fetch_single::<Session>(statement, params).await;

        result
    }
}

#[async_trait]
impl  ModifyRepository<Session> for SessionRepository {
    async fn update(&self, entity_with_update_values: &Session) -> Result<Session, DbActionError> {
        if entity_with_update_values.get_id().is_none() {
            return Err(format!("Id of entity {} is unknown; entity cannot be modified", entity_with_update_values));
        }

        let statement = format!("MATCH (u:{}) WHERE id(u) = $id SET p.refreshed = $refreshed RETURN u", Session::get_node_type_name());
        let params = Params::from_iter(vec![("id", entity_with_update_values.id.unwrap().to_string()), ("refreshed", entity_with_update_values.refreshed.to_string())]);
        
        let result = self.client.update::<Session>(statement, params).await;

        result
    }
}

#[async_trait]
impl  WriteRepository<Session> for SessionRepository {
    async fn add(&self, new_entity: &Session) -> Result<Arc<Session>, DbActionError> {
        let statement = format!("CREATE (u:{} {{session_id: $session_id, started: $started, refreshed: $refreshed }}) RETURN u", Session::get_node_type_name());
        let params = Params::from_iter(vec![("session_id", new_entity.session_id.clone()),
            ("started", new_entity.started.to_string()), ("refreshed", new_entity.refreshed.to_string())]);
        
        let client = self.client.clone();
        let result = client.create::<Session>(statement, params).await;

        if (result.is_ok()) {
            let session_mutex = Arc::new(Mutex::new(result.unwrap()));
            let user = new_entity.user.clone();
            let session_relation_result = client.create_relationship(session_mutex.clone(), user.clone(), &String::from("OWNED_BY"), None).await;
            let mut session = session_mutex.lock().unwrap();
            session.user = user;


            return session_relation_result.map(|_| Arc::new(session.clone()));
        }

        Err(result.unwrap_err())
    }

    async fn delete(&self, entity_to_delete: &Session) -> Result<(), DbActionError> {
        let result = self.client.delete::<Session>(entity_to_delete).await;

        result
    }
}