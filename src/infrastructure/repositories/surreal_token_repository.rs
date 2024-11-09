use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;

use crate::domain::entities::token::RefreshToken;
use crate::domain::repositories::token_repository::TokenRepository;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;

pub struct SurrealTokenRepository {
  db: Arc<Surreal<Client>>,
}

impl SurrealTokenRepository {
  pub fn new(connection: &DatabaseConnection) -> Self {
    Self { 
      db: connection.get_client()
    }
  }
}

#[async_trait]
impl TokenRepository for SurrealTokenRepository {
  async fn create_refresh_token(&self, refresh_token: &RefreshToken) -> Result<RefreshToken, Error> {
    let result: Option<RefreshToken> = self.db
      .query(r#"
        CREATE refresh_token SET
          token = $token,
          access_token = $access_token,
          user_id = $user_id,
          expires_at = $expires_at,
          created_at = time::now(),
          used = false,
          invalidated = false
      "#)
      .bind(("token", &refresh_token.token))
      .bind(("access_token", &refresh_token.access_token))
      .bind(("user_id", &refresh_token.user_id))
      .bind(("expires_at", &refresh_token.expires_at))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, Error> {
    let refresh_token: Option<RefreshToken> = self.db
      .query("SELECT * FROM refresh_token WHERE token = $token AND invalidated = false")
      .bind(("token", token))
      .await?
      .take(0)?;
    
    Ok(refresh_token)
  }

  async fn invalidate_refresh_token(&self, user_id: &SurrealId) -> Result<(), Error> {
    let result: Option<RefreshToken> = self.db
      .query(r#"
        UPDATE refresh_token 
        SET 
          invalidated = true,
          updated_at = time::now()
        WHERE user_id = $user_id
      "#)
      .bind(("user_id", user_id.id()))
      .await?
      .take(0)?;

    match result {
      Some(_) => Ok(()),
      None => Err(Error::InvalidToken)
    }
  }

  async fn invalidate_all_user_tokens(&self, user_id: &SurrealId) -> Result<(), Error> {
    let _: Option<RefreshToken> = self.db
      .query(r#"
        UPDATE refresh_token 
        SET
          invalidated = true,
          updated_at = time::now()
        WHERE user_id = $user_id
      "#)
      .bind(("user_id", user_id.id()))
      .await?
      .take(0)?;

    Ok(())
  }

  async fn cleanup_expired_tokens(&self) -> Result<u64, Error> {
    let result: Option<u64> = self.db
      .query(r#"
        DELETE refresh_token 
        WHERE expires_at <= time::now() 
        RETURN count()
      "#)
      .await?
      .take(0)?;

    Ok(result.unwrap_or(0))
  }
}
