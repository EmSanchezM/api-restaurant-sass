use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;

use crate::domain::entities::User;

pub struct SurrealUserRepository {
  db: Surreal<Ws>,
}

#[async_trait]
impl UserRepository for SurrealUserRepository {
  async fn create(&self, email: &str, password: &str, user_type: &str) -> Result<User, Error> {
    let result: Option<User> = self.db
        .query(r#"
            CREATE user SET 
                email = $email, 
                password = CRYPTO::ARGON2::GENERATE($password),
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

    result.ok_or(Error::UserCreationFailed)
  }
  async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error> {
    let user: Option<User> = self.db
      .query("SELECT * FROM user WHERE email = $email")
      .bind(("email", email))
      .await?
      .take(0)?;
    
    Ok(user)
  }
}