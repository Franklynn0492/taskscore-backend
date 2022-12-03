
#[cfg(test)] // Include the following only if you reun tests
use mockall::automock;

use crate::model::Entity;

type WriteActionFailedErr = String;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadRepository<E: 'static, I: 'static> where E: Entity<E, I>, I: Send + Sync {
    async fn find_by_id(&self, id: &I) -> Option<E>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadAllRepository<E: 'static, I: 'static> where E: Entity<E, I>, I: Send + Sync {
    async fn find_all(&self) -> Vec<E>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WriteRepository<E: 'static, I: 'static> where E: Entity<E, I> + Send + Sync, I: Send + Sync {
    async fn add(&self, new_entity: &E) -> Result<E, WriteActionFailedErr>;

    async fn delete(&self, entity_with_update_values: &E) -> Result<E, WriteActionFailedErr>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ModifyRepository<E: 'static, I: 'static> where E: Entity<E, I> + Send + Sync, I: Send + Sync {
    async fn update(&self, entity_with_update_values: &E) -> Result<E, WriteActionFailedErr>;
}

