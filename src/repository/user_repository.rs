use bolt_client::{Params, bolt_proto::value::Node};

use crate::{model::User};

use super::{client::{Neo4JClient, DbClient}, repository::ReadRepository};

pub struct UserRepository<'c> {
    client: &'c Neo4JClient,
}

impl <'c> UserRepository<'c> {
    pub fn new(client: &'c Neo4JClient) -> UserRepository<'c> {
        UserRepository { client }
    }

    async fn find_user_by_username_const<'a>(&'a self, username: &String) -> Option<crate::model::User> {
        let statement = "MATCH (p:Person {username: $username}) RETURN p;";
        let params = Params::from_iter(vec![("username", username.clone())]);

        return self.client.fetch_single(statement, params);
    }
}

#[async_trait]
impl <'c> ReadRepository<User, u32> for UserRepository<'c> {
    async fn find_by_id(&self, id: &u32) -> Option<User> {
        !unimplemented!()
    }
}