use bolt_client::bolt_proto::value::Node;
pub use user::User;
pub use task::{Task, Score};
pub use session::Session;


pub mod session;
pub mod user;
pub mod task;
mod util;

pub trait Entity<I: Send + Sync + 'static>: From<Node> + Send + Sync + 'static {
    fn get_id(&self) -> &I;

    fn get_node_type_name() -> &'static str;
}