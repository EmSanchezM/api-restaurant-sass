use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;

use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::domain::error::Error;

pub struct SurrealPermissionRepository {
  db: Surreal<Ws>,
}

impl SurrealPermissionRepository {
  pub fn new(db: Surreal<Ws>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl PermissionRepository for SurrealPermissionRepository {
  async fn create(&self, permission: &Permission) -> Result<Permission, Error> {
    let result: Option<Permission> = self.db
      .query(r#"
        CREATE permission SET
          name = $name,
          description = $description,
          resource = $resource,
          action = $action,
          is_active = $is_active,
          created_at = time::now()
      "#)
      .bind(("name", &permission.name))
      .bind(("description", &permission.description))
      .bind(("resource", &permission.resource))
      .bind(("action", &permission.action))
      .bind(("is_active", permission.is_active))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn find_role_permissions(&self, role_id: &str) -> Result<Vec<Permission>, Error> {
    let permissions: Vec<Permission> = self.db
      .query(r#"
          SELECT permission.* FROM role_permission
          RELATE $role_id -> permission
          WHERE permission.is_active = true
      "#)
      .bind(("role_id", role_id))
      .await?
      .take(0)?;

    Ok(permissions)
  }

  async fn assign_permission_to_role(&self, role_id: &str, permission_id: &str) -> Result<(), Error> {
    self.db
      .query(r#"
        RELATE $role_id -> role_permission -> $permission_id
      "#)
      .bind(("role_id", role_id))
      .bind(("permission_id", permission_id))
      .await?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::PermissionNotFound)
    }
  }
}
