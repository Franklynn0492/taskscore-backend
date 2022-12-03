use bolt_client::bolt_proto::value::Node;
pub use user::User;
pub use task::{Task, Score};
pub use session::Session;


pub mod session;
pub mod user;
pub mod task;

pub trait Entity<E: 'static, I: 'static> where I: Send + Sync, E: Entity<E, I> + From<Node> {
    fn get_id(&self) -> &I;
}