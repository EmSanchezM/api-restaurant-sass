use async_trait::async_trait;

use crate::domain::entities::permission::Permission;
use crate::domain::error::Error;

#[async_trait]
pub trait PermissionRepository {
  async fn find_by_id(&self, id: String) -> Result<Option<Permission>, Error>;
  async fn find_all(&self) -> Result<Vec<Permission>, Error>;
  async fn create(&self, permission: &Permission) -> Result<Permission, Error>;
  async fn update(&self, id: String, permission: &Permission) -> Result<Permission, Error>;
  async fn delete(&self, id: String) -> Result<(), Error>;
  async fn find_role_permissions(&self, role_id: String) -> Result<Vec<Permission>, Error>;
  async fn assign_permission_to_role(&self, role_id: String, permission_id: String) -> Result<(), Error>;
  async fn remove_permission_from_role(&self, role_id: String, permission_id: String) -> Result<(), Error>;
}