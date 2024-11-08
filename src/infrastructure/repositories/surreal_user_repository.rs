use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;
use surrealdb::sql::Thing as Record;

use crate::domain::entities::User;
use crate::domain::entities::role::Role;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;

pub struct SurrealUserRepository {
  db: Arc<Surreal<Client>>,
}

impl SurrealUserRepository {
  pub fn new(connection: &DatabaseConnection) -> Self {
    Self { 
      db: connection.get_client()
    }
  }
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
  async fn create(&self, user: &User) -> Result<User, Error> {
    let result: Option<User> = self.db
        .query(r#"
            LET $hashed_password = CRYPTO::ARGON2::GENERATE($password);
            CREATE type::thing($tb, $id) SET 
              email = $email, 
              password = $hashed_password,
              status = 'pending_verification',
              user_type = $user_type,
              created_at = time::now(),
              updated_at = time::now(),
              is_verified = false,
              failed_login_attempts = 0
        "#)
        .bind(("tb", user.surreal_id.table()))
        .bind(("id", user.surreal_id.id()))
        .bind(("email", &user.email))
        .bind(("password", &user.password))
        .bind(("user_type", &user.user_type))
        .await?
        .take(0)?;

    result.ok_or(Error::UserCreationError("".to_string()))
  }

  async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
    let user: Option<User> = self.db
      .query("SELECT * FROM user WHERE email = $email")
      .bind(("email", email))
      .await?
      .take(0)?;
    
    Ok(user)
  }

  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<User>, Error> {
    let user: Option<User> = self.db
      .query(r#"
        SELECT *,
        (SELECT role FROM user_role WHERE user = $id) AS roles,
        (SELECT permission FROM user_role RELATE->role_permission WHERE user_role.user = $id) AS permissions 
        FROM type::thing($tb, $id)
      "#)
      .bind(("tb", id.table()))
      .bind(("id", id.id()))
      .await?
      .take(0)?;
    
    Ok(user)
  }

  async fn delete(&self, id: &SurrealId) -> Result<(), Error> {
    let result: Option<User> = self.db
      .query(r#"
        UPDATE type::thing($tb, $id) 
        SET 
          is_active = false,
          updated_at = time::now()
      "#)
      .bind(("tb", id.table()))
      .bind(("id", id.id()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::UserNotFound)
    }
  }

  async fn update_failed_login_attempts(&self, user_id: &SurrealId, attempts: i32) -> Result<User, Error> {
    let result: Option<User> = self.db
      .query(r#"
        UPDATE type::thing($tb, $id) 
        SET 
          failed_login_attempts = $attempts,
          updated_at = time::now()
      "#)
      .bind(("tb", user_id.table()))
      .bind(("id", user_id.id()))
      .bind(("attempts", attempts))
      .await?
      .take(0)?;

    result.ok_or(Error::UserUpdateError("".to_string()))
  }

  async fn authenticate(&self, email: &str, password: &str) -> Result<Option<User>, Error> {
    let result: Option<User> = self.db
      .query(r#"
        LET $user = (SELECT * FROM user WHERE email = $email);
        IF $user.password != NONE AND CRYPTO::ARGON2::COMPARE($user.password, $password) {
          RETURN $user
        } ELSE {
          RETURN NONE
        }
      "#)
      .bind(("email", email))
      .bind(("password", password))
      .await?
      .take(0)?;
    
    Ok(result)
  }

  async fn change_password(&self, user_id: &SurrealId, new_password: &str) -> Result<User, Error> {
    let result: Option<User> = self.db
      .query(r#"
        LET $hashed_password = CRYPTO::ARGON2::GENERATE($password);
        UPDATE type::thing($tb, $id) 
        SET 
          password = $hashed_password,
          updated_at = time::now()
      "#)
      .bind(("tb", user_id.table()))
      .bind(("id", user_id.id()))
      .bind(("password", new_password))
      .await?
      .take(0)?;

    result.ok_or(Error::UserUpdateError("Updated failed".to_string()))
  }

  async fn set_verification_status(&self, user_id: &SurrealId, is_verified: bool) -> Result<User, Error> {
    let result: Option<User> = self.db
      .query(r#"
        UPDATE type::thing($tb, $id) 
        SET 
          is_verified = $is_verified,
          status = IF $is_verified == true THEN 'verified' ELSE 'pending_verification' END,
          updated_at = time::now()
      "#)
      .bind(("tb", user_id.table()))
      .bind(("id", user_id.id()))
      .bind(("is_verified", is_verified))
      .await?
      .take(0)?;

    result.ok_or(Error::UserUpdateError("Updated failed".to_string()))
  }

  async fn assign_roles(&self, user_id: &SurrealId, roles: Vec<Role>) -> Result<User, Error> {
    let _: Vec<Record> = self.db
        .query(r#"
          LET $user = type::thing($tb, $id);
          FOR $role IN $roles {
            CREATE user_role SET 
              user = $user,
              role = $role,
              assigned_at = time::now(),
              assigned_by = $user
          };
          SELECT * FROM type::thing($tb, $id);
        "#)
        .bind(("tb", user_id.table()))
        .bind(("id", user_id.id()))
        .bind(("roles", roles))
        .await?
        .take(0)?;

    self.find_by_id(user_id)
      .await?
      .ok_or(Error::UserUpdateError("Failed to assign roles".to_string()))
  }
}