
#[cfg(test)] // Include the following only if you reun tests
use mockall::automock;

use crate::model::Entity;

pub type DbActionError = String;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn find_by_id(&self, id: &I) -> Result<Option<E>, DbActionError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadAllRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn find_all(&self) -> Result<Vec<E>, DbActionError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WriteRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn add(&self, new_entity: &E) -> Result<E, DbActionError>;

    async fn delete(&self, entity_with_update_values: &E) -> Result<(), DbActionError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ModifyRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn update(&self, entity_with_update_values: &E) -> Result<E, DbActionError>;
}

