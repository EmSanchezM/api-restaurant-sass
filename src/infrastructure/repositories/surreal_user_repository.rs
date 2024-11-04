use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;

use crate::domain::entities::User;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::error::Error;

pub struct SurrealUserRepository {
  db: Surreal<Ws>,
}

impl SurrealUserRepository {
  pub fn new(db: Surreal<Ws>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
  async fn create(&self, email: &str, password: &str, user_type: &str) -> Result<User, Error> {
    let result: Option<User> = self.db
        .query(r#"
            LET $hashed_password = CRYPTO::ARGON2::GENERATE($password);
            CREATE user SET 
              email = $email, 
              password = $hashed_password,
              status = 'pending_verification',
              user_type = $user_type,
              created_at = time::now(),
              updated_at = time::now(),
              is_verified = false,
              failed_login_attempts = 0
        "#)
        .bind(("email", email))
        .bind(("password", password))
        .bind(("user_type", user_type))
        .await?
        .take(0)?;

    result.ok_or(Error::UserCreationError)
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
      .query("SELECT * FROM user WHERE id = $id")
      .bind(("id", id))
      .await?
      .take(0)?;
    
    Ok(user)
  }

  async fn delete(&self, id: &SurrealId) -> Result<(), Error> {
    let _result = self.db
      .query(r#"
        UPDATE user 
        SET 
          is_active = false,
          updated_at = time::now()
        WHERE id = $id
      "#)
      .bind(("id", id))
      .await?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::UserNotFound)
    }
  }

  async fn update_failed_login_attempts(&self, user_id: &str, attempts: i32) -> Result<User, Error> {
    let result: Option<User> = self.db
      .query(r#"
        UPDATE user 
        SET 
          failed_login_attempts = $attempts,
          updated_at = time::now()
        WHERE id = $user_id
      "#)
      .bind(("user_id", user_id))
      .bind(("attempts", attempts))
      .await?
      .take(0)?;

    result.ok_or(Error::UserUpdateError(None))
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

  async fn change_password(&self, user_id: &str, new_password: &str) -> Result<User, Error> {
    let result: Option<User> = self.db
      .query(r#"
          LET $hashed_password = CRYPTO::ARGON2::GENERATE($password);
          UPDATE user 
          SET 
            password = $hashed_password,
            updated_at = time::now()
          WHERE id = $user_id
      "#)
      .bind(("password", new_password))
      .bind(("user_id", user_id))
      .await?
      .take(0)?;

    result.ok_or(Error::UserUpdateError)
  }

  async fn set_verification_status(&self, user_id: &str, is_verified: bool) -> Result<User, Error> {
    let result: Option<User> = self.db
      .query(r#"
          UPDATE user 
          SET 
              is_verified = $is_verified,
              status = IF $is_verified == true THEN 'verified' ELSE 'pending_verification' END,
              updated_at = time::now()
          WHERE id = $user_id
      "#)
      .bind(("is_verified", is_verified))
      .bind(("user_id", user_id))
      .await?
      .take(0)?;

    result.ok_or(Error::UserUpdateError)
  }

  async fn update_failed_login_attempts(&self, user_id: &str, attempts: i32) -> Result<User, Error> {
    let result: Option<User> = self.db
      .query(r#"
        UPDATE user 
        SET 
          failed_login_attempts = $attempts,
          status = IF $attempts >= 3 THEN 'locked' ELSE status END,
          updated_at = time::now()
        WHERE id = $user_id
      "#)
      .bind(("attempts", attempts))
      .bind(("user_id", user_id))
      .await?
      .take(0)?;

    result.ok_or(Error::UserUpdateError)
  }
}