
use std::sync::Arc;

#[cfg(test)] // Include the following only if you reun tests
use mockall::automock;

use crate::model::Entity;

pub type DbActionError = String;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadRepository<E: Entity> {
    async fn find_by_id(&self, id: &E::I) -> Result<Option<E>, DbActionError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ReadAllRepository<E: Entity> {
    async fn find_all(&self) -> Result<Vec<E>, DbActionError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait WriteRepository<E: Entity> {
    async fn add(&self, new_entity: &E) -> Result<Arc<E>, DbActionError>;

    async fn delete(&self, entity_with_update_values: &E) -> Result<(), DbActionError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait ModifyRepository<E: Entity> {
    async fn update(&self, entity_with_update_values: &E) -> Result<E, DbActionError>;
}

