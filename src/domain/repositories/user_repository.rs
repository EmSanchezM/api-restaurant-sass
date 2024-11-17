use async_trait::async_trait;

use crate::domain::entities::user::User;
use crate::domain::entities::role::Role;
use crate::domain::error::Error;

#[async_trait]
pub trait UserRepository {
  async fn create(&self, user: &User) -> Result<User, Error>;
  async fn find_by_id(&self, id: String) -> Result<Option<User>, Error>;
  async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
  async fn delete(&self, id: String) -> Result<(), Error>;
  async fn change_password(&self, user_id: String, new_password: &str) -> Result<User, Error>;
  async fn set_verification_status(&self, user_id: String, is_verified: bool) -> Result<User, Error>;
  async fn update_failed_login_attempts(&self, user_id: String, attempts: i32) -> Result<User, Error>;
  async fn authenticate(&self, email: &str, password: &str) -> Result<Option<User>, Error>;
  async fn assign_roles(&self, user_id: String, roles: Vec<Role>) -> Result<User, Error>;
}