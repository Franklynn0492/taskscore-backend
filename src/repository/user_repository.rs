use bolt_client::{Params};

use crate::{model::User};

use super::{client::{Neo4JClient, DbClient}, repository::{ReadRepository, DbActionError, ModifyRepository}};

pub struct UserRepository<'c> {
    client: &'c Neo4JClient,
}

impl <'c> UserRepository<'c> {
    pub fn new(client: &'c Neo4JClient) -> UserRepository<'c> {
        UserRepository { client }
    }

    async fn find_user_by_username_const<'a>(&'a self, username: &String) -> Result<Option<crate::model::User>, DbActionError> {
        let statement = "MATCH (p:Person {username: $username}) RETURN p;";
        let params = Params::from_iter(vec![("username", username.clone())]);

        return self.client.fetch_single::<User, u32>(statement, params).await;
    }
}

#[async_trait]
impl <'c> ReadRepository<User, u32> for UserRepository<'c> {
    async fn find_by_id(&self, id: &u32) -> Result<Option<crate::model::User>, DbActionError> {
        let statement = "MATCH (p:Person) WHERE id(p) = $id RETURN p;";
        let params = Params::from_iter(vec![("id", id.to_string())]);

        return self.client.fetch_single::<User, u32>(statement, params).await;
    }
}

#[async_trait]
impl <'c> ModifyRepository<User, u32> for UserRepository<'c> {
    async fn update(&self, entity_with_update_values: &User) -> Result<User, DbActionError> {
        let statement = "MATCH (p:Person) WHERE id(p) = $id SET p.display_name = '$display_name', p.password = '$password', p.username = '$username' RETURN p";
        let params = Params::from_iter(vec![("id", entity_with_update_values.id.to_string()), ("display_name", entity_with_update_values.display_name),
            ("password", entity_with_update_values.pwd_hash_components.unwrap_or("".to_owned())), ("username", entity_with_update_values.username)]);
    }
}

