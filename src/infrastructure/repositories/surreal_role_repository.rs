use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;

use crate::domain::entities::role::Role;
use crate::domain::repositories::role_repository::RoleRepository;
use crate::domain::error::Error;

pub struct SurrealRoleRepository {
  db: Surreal<Ws>,
}

impl SurrealRoleRepository {
  pub fn new(db: Surreal<Ws>) -> Self {
    Self { db }
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
          is_active = $is_active,
          created_at = time::now()
      "#)
      .bind(("name", &role.name))
      .bind(("description", &role.description))
      .bind(("hierarchy_level", role.hierarchy_level))
      .bind(("is_active", role.is_active))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn find_user_roles(&self, user_id: &str) -> Result<Vec<Role>, Error> {
    let roles: Vec<Role> = self.db
      .query(r#"
        SELECT role.* FROM user_role
        RELATE $user_id -> role
        WHERE role.is_active = true
      "#)
      .bind(("user_id", user_id))
      .await?
      .take(0)?;

    Ok(roles)
  }

  async fn assign_role_to_user(&self, user_id: &str, role_id: &str) -> Result<(), Error> {
    self.db
      .query(r#"
          RELATE $user_id -> user_role -> $role_id SET
            assigned_at = time::now()
      "#)
      .bind(("user_id", user_id))
      .bind(("role_id", role_id))
      .await?;

    Ok(())
  }
}