use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;

use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::RoleRepository;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;

pub struct SurrealRoleRepository {
  db: Arc<Surreal<Client>>,
}

impl SurrealRoleRepository {
  pub fn new(connection: &DatabaseConnection) -> Self {
    Self { 
      db: connection.get_client()
    }
  }
}

#[async_trait]
impl RoleRepository for SurrealRoleRepository {
  async fn create(&self, role: &Role) -> Result<Role, Error> {
    let result: Option<Role> = self.db
      .query(r#"
        CREATE role SET
          name = $name,
          description = $description,
          hierarchy_level = $hierarchy_level,
          is_active = true,
          created_at = time::now()
      "#)
      .bind(("name", &role.name))
      .bind(("description", &role.description))
      .bind(("hierarchy_level", role.hierarchy_level))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn find_user_roles(&self, user_id: &SurrealId) -> Result<Vec<Role>, Error> {
    let roles: Vec<Role> = self.db
      .query(r#"
        SELECT role.* FROM user_role
        RELATE type::thing("user", $user_id) -> role
        WHERE role.is_active = true
      "#)
      .bind(("user_id", user_id.id().to_string()))
      .await?
      .take(0)?;

    Ok(roles)
  }

  async fn assign_role_to_user(&self, user_id: &SurrealId, role_id: &SurrealId) -> Result<(), Error> {
    let result: Option<()> = self.db
      .query(r#"
        RELATE type::thing("user", $user_id) -> user_role -> type::thing("role", $role_id) SET
          assigned_at = time::now()
      "#)
      .bind(("user_id", user_id.id().to_string()))
      .bind(("role_id", role_id.id().to_string()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::RoleNotFound)
    }
  }

  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Role>, Error> {
    let role: Option<Role> = self.db
      .query(r#"
        SELECT * FROM role WHERE id = type::thing("role", $id) AND is_active = true
      "#)
      .bind(("id", id.id().to_string()))
      .await?
      .take(0)?;

    Ok(role)
  }

  async fn find_all(&self) -> Result<Vec<Role>, Error> {
    let roles: Vec<Role> = self.db
      .query("SELECT * FROM role WHERE is_active = true")
      .await?
      .take(0)?;

    Ok(roles)
  }

  async fn update(&self, id: &SurrealId, role: &Role) -> Result<Role, Error> {
    let result: Option<Role> = self.db
      .query(r#"
        UPDATE type::thing($tb, $id) SET
          name = $name,
          description = $description,
          hierarchy_level = $hierarchy_level
      "#)
      .bind(("tb", id.table()))
      .bind(("id", id.id()))
      .bind(("name", &role.name))
      .bind(("description", &role.description))
      .bind(("hierarchy_level", role.hierarchy_level))
      .await?
      .take(0)?;

    result.ok_or(Error::RoleNotFound)
  }

  async fn delete(&self, id: &SurrealId) -> Result<(), Error> {
    let result: Option<Role> = self.db
      .query(r#"
        DELETE role WHERE id = type::thing("role", $id)
      "#)
      .bind(("id", id.id().to_string()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::RoleNotFound)
    }
  }

  async fn remove_role_from_user(&self, user_id: &SurrealId, role_id: &SurrealId) -> Result<(), Error> {
    let result: Option<Role> = self.db
      .query(r#"
        DELETE RELATE type::thing("user", $user_id) -> user_role -> type::thing("role", $role_id)
      "#)
      .bind(("user_id", user_id.id().to_string()))
      .bind(("role_id", role_id.id().to_string()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::RoleNotFound)
    }
  }
}