use async_trait::async_trait;

use crate::domain::entities::user::User;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::error::Error;

#[async_trait]
pub trait UserRepository {
  async fn create(&self, user: &User) -> Result<User, Error>;
  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<User>, Error>;
  async fn find_by_email(&self, email: &Email) -> Result<Option<User>, Error>;
  async fn delete(&self, id: &SurrealId) -> Result<(), Error>;
  async fn change_password(&self, user_id: &str, new_password: &str) -> Result<User, Error>;
  async fn set_verification_status(&self, user_id: &str, is_verified: bool) -> Result<User, Error>;
  async fn update_failed_login_attempts(&self, user_id: &str, attempts: i32) -> Result<User, Error>;
}
