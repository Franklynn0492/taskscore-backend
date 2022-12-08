use std::fmt::Display;

use bolt_client::bolt_proto::value::Node;
pub use user::User;
pub use task::{Task, Score};
pub use session::Session;


pub mod session;
pub mod user;
pub mod task;
mod util;

pub trait Entity<Id>: From<Node> + Send + Sync + 'static {
    fn get_id(&self) -> &Id;

    fn get_node_type_name() -> &'static str;
}

pub trait Id: Send + Sync + 'static + Display {}

// TODO: check if this can be improved/avoided
impl Id for u32 {}