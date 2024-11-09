use async_trait::async_trait;

use crate::domain::entities::permission::Permission;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;
#[async_trait]
pub trait PermissionRepository {
  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Permission>, Error>;
  async fn find_all(&self) -> Result<Vec<Permission>, Error>;
  async fn create(&self, permission: &Permission) -> Result<Permission, Error>;
  async fn update(&self, id: &SurrealId, permission: &Permission) -> Result<Permission, Error>;
  async fn delete(&self, id: &SurrealId) -> Result<(), Error>;
  async fn find_role_permissions(&self, role_id: &SurrealId) -> Result<Vec<Permission>, Error>;
  async fn assign_permission_to_role(&self, role_id: &SurrealId, permission_id: &SurrealId) -> Result<(), Error>;
  async fn remove_permission_from_role(&self, role_id: &SurrealId, permission_id: &SurrealId) -> Result<(), Error>;
}