use async_trait::async_trait;

use crate::domain::entities::role::Role;

#[async_trait]
pub trait RoleRepository {
  async fn find_by_id(&self, id: &str) -> Result<Option<Role>, Error>;
  async fn find_all(&self) -> Result<Vec<Role>, Error>;
  async fn create(&self, role: &Role) -> Result<Role, Error>;
  async fn update(&self, role: &Role) -> Result<Role, Error>;
  async fn delete(&self, id: &str) -> Result<(), Error>;
  async fn find_user_roles(&self, user_id: &str) -> Result<Vec<Role>, Error>;
  async fn assign_role_to_user(&self, user_id: &str, role_id: &str) -> Result<(), Error>;
  async fn remove_role_from_user(&self, user_id: &str, role_id: &str) -> Result<(), Error>;
}