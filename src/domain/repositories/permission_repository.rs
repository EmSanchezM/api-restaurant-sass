use async_trait::async_trait;

use crate::domain::entities::permission::Permission;

#[async_trait]
pub trait PermissionRepository {
  async fn find_by_id(&self, id: &str) -> Result<Option<Permission>, Error>;
  async fn find_all(&self) -> Result<Vec<Permission>, Error>;
  async fn create(&self, permission: &Permission) -> Result<Permission, Error>;
  async fn update(&self, permission: &Permission) -> Result<Permission, Error>;
  async fn delete(&self, id: &str) -> Result<(), Error>;
  async fn find_role_permissions(&self, role_id: &str) -> Result<Vec<Permission>, Error>;
  async fn assign_permission_to_role(&self, role_id: &str, permission_id: &str) -> Result<(), Error>;
  async fn remove_permission_from_role(&self, role_id: &str, permission_id: &str) -> Result<(), Error>;
}