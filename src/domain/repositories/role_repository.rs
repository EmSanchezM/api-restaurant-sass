use async_trait::async_trait;

use crate::domain::error::Error;
use crate::domain::entities::role::Role;
use crate::domain::value_objects::surreal_id::SurrealId;
#[async_trait]
pub trait RoleRepository {
  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Role>, Error>;
  async fn find_all(&self) -> Result<Vec<Role>, Error>;
  async fn create(&self, role: &Role) -> Result<Role, Error>;
  async fn update(&self, id: &SurrealId, role: &Role) -> Result<Role, Error>;
  async fn delete(&self, id: &SurrealId) -> Result<(), Error>;
  async fn find_user_roles(&self, user_id: &SurrealId) -> Result<Vec<Role>, Error>;
  async fn assign_role_to_user(&self, user_id: &SurrealId, role_id: &SurrealId) -> Result<(), Error>;
  async fn remove_role_from_user(&self, user_id: &SurrealId, role_id: &SurrealId) -> Result<(), Error>;
}