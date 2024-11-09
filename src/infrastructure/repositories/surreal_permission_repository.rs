use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;

use crate::domain::entities::permission::Permission;
use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::error::Error;

use crate::infrastructure::database::surreal_connection::DatabaseConnection;

pub struct SurrealPermissionRepository {
  db: Arc<Surreal<Client>>,
}

impl SurrealPermissionRepository {
  pub fn new(connection: &DatabaseConnection) -> Self {
    Self { 
      db: connection.get_client()
    }
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
          is_active = true,
          created_at = time::now()
      "#)
      .bind(("name", &permission.name))
      .bind(("description", &permission.description))
      .bind(("resource", &permission.resource))
      .bind(("action", &permission.action))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn find_role_permissions(&self, role_id: &SurrealId) -> Result<Vec<Permission>, Error> {
    let permissions: Vec<Permission> = self.db
      .query(r#"
        SELECT permission.* FROM role_permission
        RELATE type::thing("role", $role_id) -> role_permission -> permission
        WHERE permission.is_active = true
      "#)
      .bind(("role_id", role_id.id().to_string()))
      .await?
      .take(0)?;

    Ok(permissions)
  }

  async fn assign_permission_to_role(&self, role_id: &SurrealId, permission_id: &SurrealId) -> Result<(), Error> {
    let result: Option<()> = self.db
      .query(r#"
        RELATE $role_id -> role_permission -> $permission_id
      "#)
      .bind(("role_id", role_id.id().to_string()))
      .bind(("permission_id", permission_id.id().to_string()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::PermissionNotFound)
    }
  }

  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Permission>, Error> {
    let result: Option<Permission> = self.db
      .query(r#"
        SELECT * FROM permission 
        WHERE id = type::thing("permission", $id) AND is_active = true
      "#)
      .bind(("id", id.id().to_string()))
      .await?
      .take(0)?;
    
    Ok(result)
  }

  async fn find_all(&self) -> Result<Vec<Permission>, Error> {
    let permissions: Vec<Permission> = self.db
      .query(r#"
        SELECT * FROM permission 
        WHERE is_active = true
      "#)
      .await?
      .take(0)?;
    
    Ok(permissions)
  }

  async fn update(&self, id: &SurrealId, permission: &Permission) -> Result<Permission, Error> {
    let result: Option<Permission> = self.db
      .query(r#" 
        UPDATE permission 
        SET name = $name, description = $description, resource = $resource, action = $action 
        WHERE id = type::thing("permission", $id) AND is_active = true
      "#)
      .bind(("id", id.id().to_string()))
      .bind(("name", &permission.name))
      .bind(("description", &permission.description))
      .bind(("resource", &permission.resource))
      .bind(("action", &permission.action))
      .await?
      .take(0)?;
    
    result.ok_or(Error::PermissionNotFound)
  }

  async fn delete(&self, id: &SurrealId) -> Result<(), Error> {
    let result: Option<Permission> = self.db
      .query(r#"
        UPDATE permission SET is_active = false 
        WHERE id = type::thing("permission", $id) AND is_active = true
      "#)
      .bind(("id", id.id().to_string()))
      .await?
      .take(0)?;
    
    match result {
      Some(_) => Ok(()),
      None => Err(Error::PermissionNotFound)
    }
  }

  async fn remove_permission_from_role(&self, role_id: &SurrealId, permission_id: &SurrealId) -> Result<(), Error> {
    let result: Option<()> = self.db
      .query(r#"
        DELETE FROM role_permission 
        WHERE in = type::thing("role", $role_id) 
        AND out = type::thing("permission", $permission_id)
      "#)
      .bind(("role_id", role_id.id().to_string()))
      .bind(("permission_id", permission_id.id().to_string()))
      .await?
      .take(0)?;
    
    match result {
      Some(_) => Ok(()),
      None => Err(Error::PermissionNotFound)
    }
  }
}
