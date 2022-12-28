use std::{fmt::{Display, Debug, self}, sync::{Arc, Mutex}, collections::HashMap, rc::Rc};

use bolt_client::bolt_proto::{value::{Node, Relationship}, Value};
pub use user::User;
pub use task::{Task, Score};
pub use session::Session;

use crate::repository::repository::ReadRepository;


pub mod session;
pub mod user;
pub mod task;
mod util;

pub type FromInput = (HashMap<String, Vec<Node>>, HashMap<String, Vec<Relationship>>);

pub trait Entity: TryFrom<FromInput> + From<Node> + Send + Sync + 'static + Display + Debug {
    type I: Id;

    fn get_id(&self) -> &Option<Self::I>;

    fn get_node_type_name() -> &'static str;
    
    fn fmt_entity(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id_str = if self.get_id().is_none() {
            "none".to_string()
        } else {
            self.get_id().as_ref().unwrap().to_string()
        };

        write!(f, "[{}; id: {})]", Self::get_node_type_name(), id_str)
    }
}

pub trait Id: Send + Sync + 'static + Display + TryFrom<i64> + Into<i64> + Clone {}

impl Id for u32 {
}

#[derive(Debug)]
pub struct Relation<S: Entity, T: Entity> {
    source_node: Arc<Mutex<S>>,
    target_node: Arc<Mutex<T>>,
    name: String,
    params_opt: Option<HashMap<String, Value>>,
}

impl <S: Entity, T: Entity> Relation<S, T> {
    pub fn new(source_node: Arc<Mutex<S>>, target_node: Arc<Mutex<T>>, name: String, params_opt: Option<HashMap<String, Value>>) -> Result<Relation<S, T>, String> {

        if source_node.lock().unwrap().get_id().is_none() || target_node.lock().unwrap().get_id().is_none() {
            Err(format!("Both nodes need to have an Id when creating a relation; source_node.id: {}; target_node.id: {}", source_node.lock().unwrap(), target_node.lock().unwrap()))
        } else {
            Ok(Relation { source_node: source_node, target_node, name, params_opt })
        }
    }

    pub fn get_create_statement(&self) -> String {
        let statement = format!("MATCH (s:{}), (t:{}) WHERE id(s) = {} AND id(t) = {} CREATE (s)-[r:{} {}]->(t) RETURN r",
        S::get_node_type_name(), T::get_node_type_name(), self.source_node.lock().unwrap().get_id().as_ref().unwrap(), self.target_node.lock().unwrap().get_id().as_ref().unwrap(), self.name, self.params_to_str());
        statement
    }

    pub fn get_match_statement(&self) -> String {
        let statement = format!("MATCH (s:{}) -[r:{}]- (t:{}) WHERE id(s) = {} AND id(t) = {} RETURN r",
        S::get_node_type_name(), self.name, T::get_node_type_name(), self.source_node.lock().unwrap().get_id().as_ref().unwrap(), self.target_node.lock().unwrap().get_id().as_ref().unwrap());
        statement
    }

    pub fn get_delete_statement(&self) -> String {
        let statement = format!("MATCH (s:{}) -[r:{}]- (t:{}) WHERE id(s) = {} AND id(t) = {} DELETE r",
        S::get_node_type_name(), self.name, T::get_node_type_name(), self.source_node.lock().unwrap().get_id().as_ref().unwrap(), self.target_node.lock().unwrap().get_id().as_ref().unwrap());
        statement
    }

    fn params_to_str(&self) -> String  {
        if self.params_opt.is_none() {
            return String::new();
        }

        // Unfortunately I have to clone here. Unwrapping takes ownership (and we cannot own self.params_opt here), and .as_deref() does
        // not work since Value is not Deref-erable
        let param_str = self.params_opt.clone().unwrap().iter()
            .map(|(k, v)| Self::try_param_pair_to_str(k, v))
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect::<Vec<String>>()
            .join(", ");
        
        param_str
    }

    fn try_param_pair_to_str(key: &String, value: &Value) -> Result<String, ()> {
        let value_str_opt = match value {
            Value::Boolean(v) => Some(v.to_string()),
            Value::Date(v) => Some(v.to_string()),
            Value::DateTimeOffset(v) => Some(v.to_string()),
            Value::DateTimeZoned(v) => Some(v.to_string()),
            Value::Float(v) => Some(v.to_string()),
            Value::Integer(v) => Some(v.to_string()),
            Value::LocalDateTime(v) => Some(v.to_string()),
            Value::LocalTime(v) => Some(v.to_string()),
            Value::Null => Some("null".to_string()),
            Value::String(v) => Some(v.clone()),
            _ => None
        };

        if value_str_opt.is_none() {
            Err(())
        } else {
            let result = format!("{}: {}", key, value_str_opt.unwrap());
            Ok(result)
        }
    }


    pub async fn try_from(value: &Relationship, source_repository: &dyn ReadRepository<S>, target_repository: &dyn ReadRepository<T>) -> Result<Self, String> {
        let src_id = value.start_node_identity();
        let target_id = value.end_node_identity().try_into().unwrap();
        let properties = value.properties();
        let name = value.rel_type();

        let source_res = Self::resolve_by_id(source_repository, &src_id).await;
        let target_res = Self::resolve_by_id(target_repository, &target_id).await;

        if source_res.is_err() || target_res.is_err() {
            return Err(format!("Could not load relation of type {}; Source is present: {}; target is present: {}; src_id: {}, target_id: {}", name, source_res.is_ok(), target_res.is_ok(), src_id, target_id));
        }

        let params_opt = if properties.len() == 0 { None } else {
            Some(properties.clone())
        };

        let relation = Relation::new(Arc::new(Mutex::new(source_res.unwrap())), Arc::new(Mutex::new(target_res.unwrap())), String::from(name), params_opt);

        relation
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