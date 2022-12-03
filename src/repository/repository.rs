
#[cfg(test)] // Include the following only if you reun tests
use mockall::automock;

use crate::model::Entity;

type WriteActionFailedErr = String;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn find_by_id(&self, id: &I) -> Option<E>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadAllRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn find_all(&self) -> Vec<E>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WriteRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn add(&self, new_entity: &E) -> Result<E, WriteActionFailedErr>;

    async fn delete(&self, entity_with_update_values: &E) -> Result<E, WriteActionFailedErr>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ModifyRepository<E, I: Send + Sync + 'static> where E: Entity<I> {
    async fn update(&self, entity_with_update_values: &E) -> Result<E, WriteActionFailedErr>;
}

