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

pub trait Entity: From<Node> + Send + Sync + 'static + Display + Debug {
    type I: Id;

    fn get_id(&self) -> Option<&Self::I>;

    fn get_node_type_name() -> &'static str;
    
    fn fmt_entity(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id_str = if self.get_id().is_none() {
            "none".to_string()
        } else {
            self.get_id().unwrap().to_string()
        };

        write!(f, "[{}; id: {})]", Self::get_node_type_name(), id_str)
    }
}

pub trait Id: Send + Sync + 'static + Display + TryFrom<i64> {}

impl Id for u32 {
}

#[derive(Debug)]
pub struct Relation<S: Entity, T: Entity> {
    source_node: Arc<S>,
    target_node: Arc<T>,
    name: &'static str,
    params_opt: Option<HashMap<&'static str, String>>,
}

impl <S: Entity, T: Entity> Relation<S, T> {
    pub fn new(source_node: Arc<S>, target_node: Arc<T>, name: &'static str, params_opt: Option<HashMap<&'static str, String>>) -> Result<Relation<S, T>, String> {
        if source_node.get_id().is_none() || target_node.get_id().is_none() {
            Err(format!("Both nodes need to have an Id when creating a relation; source_node.id: {}; target_node.id: {}", source_node, target_node))
        } else {
            Ok(Relation { source_node: source_node.clone(), target_node: target_node.clone(), name, params_opt })
        }
    }

    pub fn get_create_statement(&self) -> String {
        let statement = format!("MATCH (s:{}), (t:{}) WHERE id(s) = {} AND id(t) = {} CREATE (s)-[r:{} {}]->(t) RETURN r",
        S::get_node_type_name(), T::get_node_type_name(), self.source_node.get_id().unwrap(), self.target_node.get_id().unwrap(), self.name, self.params_to_str());
        statement
    }

    pub fn get_match_statement(&self) -> String {
        let statement = format!("MATCH (s:{}) -[r:{}]- (t:{}) WHERE id(s) = {} AND id(t) = {} RETURN r",
        S::get_node_type_name(), self.name, T::get_node_type_name(), self.source_node.get_id().unwrap(), self.target_node.get_id().unwrap());
        statement
    }

    pub fn get_delete_statement(&self) -> String {
        let statement = format!("MATCH (s:{}) -[r:{}]- (t:{}) WHERE id(s) = {} AND id(t) = {} DELETE r",
        S::get_node_type_name(), self.name, T::get_node_type_name(), self.source_node.get_id().unwrap(), self.target_node.get_id().unwrap());
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


    pub async fn try_from(value: &Relationship, source_repository: &dyn ReadRepository<S>, target_repository: &dyn ReadRepository<T>) -> Result<Self, String> {
        let src_id = value.start_node_identity();
        let target_id = value.end_node_identity().try_into().unwrap();
        let properties = value.properties();

        let source_res = Self::resolve_by_id(source_repository, &src_id).await;
        let target_res = Self::resolve_by_id(target_repository, &target_id).await;




        !unimplemented!()
    }

    async fn resolve_by_id<E: Entity>(repository: &dyn ReadRepository<E>, id: &i64) -> Result<E, String> {
        let entity_id_res = (*id).try_into();
        let result = match entity_id_res {
            Err(_) => Err(format!("Id {} stored in relationship to entity {} is not convertible to target id", id, E::get_node_type_name())),
            Ok(entity_id) => {

                let entity_result = repository.find_by_id(&entity_id).await;
                match entity_result {
                    Err(s) => Err(s),
                    Ok(None) => Err(format!("Unable to create Relation; could not find entity of type {} with id {}", E::get_node_type_name(), id)),
                    Ok(Some(e)) => Ok(e)
                }
            }
        };

        result

    }
}