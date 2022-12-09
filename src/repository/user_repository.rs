use std::{rc::Rc, sync::Arc};

use bolt_client::{Params};

use crate::{model::User};

use super::{client::{Neo4JClient, DbClient}, repository::{ReadRepository, DbActionError, ModifyRepository, WriteRepository, ReadAllRepository}};

pub struct UserRepository {
    client: Arc<Neo4JClient>,
}

impl  UserRepository {
    pub fn new(client:  Arc<Neo4JClient>) -> UserRepository {
        UserRepository { client }
    }

    async fn find_user_by_username_const<'a>(&'a self, username: &String) -> Result<Option<crate::model::User>, DbActionError> {
        let statement = "MATCH (p:Person {username: $username}) RETURN p;".to_owned();
        let params = Params::from_iter(vec![("username", username.clone())]);
        // storing result for debug reasons
        let result = self.client.fetch_single::<User, u32>(statement, params).await;

        result
    }
}

#[async_trait]
impl  ReadRepository<User, u32> for UserRepository {
    async fn find_by_id(&self, id: &u32) -> Result<Option<crate::model::User>, DbActionError> {
        let result = self.client.fetch_by_id::<User, u32>(&id).await;

        result
    }
}

#[async_trait]
impl  ModifyRepository<User, u32> for UserRepository {
    async fn update(&self, entity_with_update_values: &User) -> Result<User, DbActionError> {
        let statement = "MATCH (p:Person) WHERE id(p) = $id SET p.display_name = '$display_name', p.password = '$password', p.username = '$username' RETURN p".to_owned();
        let params = Params::from_iter(vec![("id", entity_with_update_values.id.to_string()), ("display_name", entity_with_update_values.display_name.clone()),
            ("password", entity_with_update_values.pwd_hash_components.clone().unwrap_or("".to_owned())), ("username", entity_with_update_values.username.clone())]);
        
        let result = self.client.update::<User, u32>(statement, params).await;

        result
    }
}

#[async_trait]
impl  WriteRepository<User, u32> for UserRepository {
    async fn add(&self, new_entity: &User) -> Result<User, DbActionError> {
        let statement = "CREATE (p:Person {username: '$username', display_name: '$display_name', password: '$password', is_admin: &is_admin }) RETURN p".to_owned();
        let params = Params::from_iter(vec![("username", new_entity.username.clone()), ("display_name", new_entity.display_name.clone()),
            ("password", new_entity.pwd_hash_components.clone().unwrap_or("".to_owned())), ("is_admin", new_entity.is_admin.to_string())]);
        
        let result = self.client.create::<User, u32>(statement, params).await;

        result
    }

    async fn delete(&self, entity_to_delete: &User) -> Result<(), DbActionError> {
        let result = self.client.delete::<User, u32>(entity_to_delete).await;

        result
    }
}

#[async_trait]
impl  ReadAllRepository<User, u32> for UserRepository {
    async fn find_all(&self) -> Result<Vec<User>, DbActionError> {
        let result = self.client.fetch_all::<User, u32>().await;

        result
    }
}

