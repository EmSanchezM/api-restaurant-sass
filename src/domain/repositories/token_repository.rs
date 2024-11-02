use async_trait::async_trait;

use crate::domain::entities::token::RefreshToken;
use crate::domain::value_objects::surreal_id::SurrealId;

#[async_trait]
pub trait TokenRepository {
  async fn create_refresh_token(&self, refresh_token: &RefreshToken) -> Result<RefreshToken, Error>;
  async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, Error>;
  async fn invalidate_refresh_token(&self, token: &str) -> Result<(), Error>;
  async fn invalidate_all_user_tokens(&self, user_id: &SurrealId) -> Result<(), Error>;
  async fn cleanup_expired_tokens(&self) -> Result<u64, Error>;
}