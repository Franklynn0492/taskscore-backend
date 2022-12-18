use std::{fmt::{Display, Debug, self}, sync::Arc, collections::HashMap};

use bolt_client::bolt_proto::value::{Node, Relationship};
pub use user::User;
pub use task::{Task, Score};
pub use session::Session;

use crate::repository::repository::ReadRepository;


pub mod session;
pub mod user;
pub mod task;
mod util;

pub trait Entity<Id: ?Sized>: From<Node> + Send + Sync + 'static + Display {
    
    fn get_id(&self) -> Option<&Id>;

    fn get_node_type_name() -> &'static str;
    
    fn fmt_entity(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id_str = if self.get_id().is_none() {
            "none".to_string()
        } else {
            self.get_id().unwrap()
        };

        write!(f, "[{}; id: {})]", Entity::get_node_type_name(), id_str)
    }
}

pub trait Id: Send + Sync + 'static + Display  {}

// TODO: check if this can be improved/avoided
impl Id for u32 {}

pub struct Relation<S: Entity<IS>, IS: Id, T: Entity<IT>, IT: Id> {
    source_node: Arc<S>,
    target_node: Arc<T>,
    name: &'static str,
    params_opt: Option<HashMap<&'static str, String>>,
}

impl <S: Entity<IS>, IS: Id, T: Entity<IT>, IT: Id> Relation<S, IS, T, IT> {
    pub fn new(source_node: Arc<S>, target_node: Arc<T>, name: &'static str, params_opt: Option<HashMap<&'static str, String>>) -> Result<Relation<S, IS, T, IT>, String> {
        if source_node.get_id().is_none() || target_node.get_id().is_none() {
            Err(format!("Both nodes need to have an Id when creating a relation; source_node.id: {}; target_node.id: {}", source_node.get_id(), target_node.get_id()))
        } else {
            Ok(Relation { source_node: source_node.clone(), target_node: target_node.clone(), name, params_opt })
        }
    }

    pub fn get_create_statement(&self) -> String {
        // Pattern: MATCH (u:User) MATCH (t:Task) WHERE id(u) = 2 AND id(t) = 4 MERGE (u)-[:SCORED {points: 11, scored_at: localdatetime()}] -> (t)
        let statement = format!("MATCH (s:{}) MATCH (t:{}) WHERE id(s) = {} AND id(t) = {} MERGE (s)-[:{} {}]->(t) RETURN *",
        S::get_node_type_name(), T::get_node_type_name(), self.source_node.get_id().unwrap(), self.target_node.get_id().unwrap(), self.name, self.params_to_str());
        statement
    }

    fn params_to_str(&self) -> String  {
        let param_str = 
        if self.params_opt.is_some() {
            let params = self.params_opt.unwrap();
            serde_json::to_string(&params).unwrap()
        } else {
            String::new()
        };
        
        param_str
    }


    pub async fn try_from(value: &Relationship, source_repository: &dyn ReadRepository<S, IS>, target_repository: &dyn ReadRepository<T, IT>) -> Result<Self, String> {
        let src_id = value.start_node_identity() as IS;
        let target_id = value.end_node_identity() as IT;
        let properties = value.properties();

        let source_res = Self::resolve_by_id(source_repository, &src_id).await;
        let target_res = Self::resolve_by_id(target_repository, &target_id).await;




        !unimplemented!()
    }

    async fn resolve_by_id<E: Entity<I>, I: Id>(repository: &dyn ReadRepository<E, I>, id: &I) -> Result<E, String> {
        let entity_result = repository.find_by_id(id).await;
        match entity_result {
            Err(S) => Err(s),
            Ok(None) => Err(format!("Unable to create Relation; could not find entity of type {} with id {}", E::get_node_type_name(), id)),
            Ok(Some(e)) => Some(e)
        }
    }
}