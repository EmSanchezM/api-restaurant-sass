use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;

use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::RoleRepository;
use crate::domain::error::Error;

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
    let result: Option<Role> = self.db.create("roles").content(role.clone()).await?;

    result.ok_or(Error::CreationFailed)
  }

  async fn find_user_roles(&self, user_id: String) -> Result<Vec<Role>, Error> {
    let roles: Vec<Role> = self.db
      .query(r#"
        SELECT role.* FROM user_role
        RELATE type::thing("user", $user_id) -> role
        WHERE role.is_active = true
      "#)
      .bind(("user_id", user_id.clone().to_string()))
      .await?
      .take(0)?;

    Ok(roles)
  }

  async fn assign_role_to_user(&self, user_id: String, role_id: String) -> Result<(), Error> {
    let result: Option<()> = self.db
      .query(r#"
        RELATE type::thing("users", $user_id) -> user_role -> type::thing("roles", $role_id) SET
          assigned_at = time::now()
      "#)
      .bind(("user_id", user_id.clone()))
      .bind(("role_id", role_id.clone()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::RoleNotFound)
    }
  }

  async fn find_by_id(&self, id: String) -> Result<Option<Role>, Error> {
    let role: Option<Role> = self.db
      .query(r#"
        SELECT * FROM roles WHERE id = type::thing("roles", $id) AND is_active = true
      "#)
      .bind(("id", id.clone()))
      .await?
      .take(0)?;

    Ok(role)
  }

  async fn find_all(&self) -> Result<Vec<Role>, Error> {
    let roles: Vec<Role> = self.db
      .query("SELECT * FROM roles WHERE is_active = true")
      .await?
      .take(0)?;

    Ok(roles)
  }

  async fn update(&self, id: String, role: &Role) -> Result<Role, Error> {
    let result: Option<Role> = self.db.update(("roles", id)).content(role.clone()).await?;

    result.ok_or(Error::RoleNotFound)
  }

  async fn delete(&self, id: String) -> Result<(), Error> {
    let result: Option<Role> = self.db
      .query(r#"
        UPDATE type::thing($tb, $id) SET
          is_active = false
      "#)
      .bind(("tb", "roles"))
      .bind(("id", id.clone()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::RoleNotFound)
    }
  }

  async fn remove_role_from_user(&self, user_id: String, role_id: String) -> Result<(), Error> {
    let result: Option<Role> = self.db
      .query(r#"
        DELETE RELATE type::thing("users", $user_id) -> user_role -> type::thing("roles", $role_id)
      "#)
      .bind(("user_id", user_id.clone()))
      .bind(("role_id", role_id.clone()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::RoleNotFound)
    }
  }
}