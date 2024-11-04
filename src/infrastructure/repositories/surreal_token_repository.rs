use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;

use crate::domain::entities::token::RefreshToken;
use crate::domain::repositories::token_repository::TokenRepository;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;

pub struct SurrealTokenRepository {
  db: Surreal<Ws>,
}


impl SurrealTokenRepository {
  pub fn new(db: Surreal<Ws>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl TokenRepository for SurrealTokenRepository {
  async fn create_refresh_token(&self, refresh_token: &RefreshToken) -> Result<RefreshToken, Error> {
    let result: Option<RefreshToken> = self.db
      .query(r#"
        CREATE refresh_token SET
          token = $token,
          user_id = $user_id,
          expires_at = $expires_at,
          created_at = time::now(),
          is_valid = true
      "#)
      .bind(("token", &refresh_token.token))
      .bind(("user_id", &refresh_token.user_id))
      .bind(("expires_at", &refresh_token.expires_at))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn find_refresh_token(&self, token: &str) -> Result<Option<RefreshToken>, Error> {
    let refresh_token: Option<RefreshToken> = self.db
      .query("SELECT * FROM refresh_token WHERE token = $token AND is_valid = true")
      .bind(("token", token))
      .await?
      .take(0)?;
    
    Ok(refresh_token)
  }

  async fn invalidate_refresh_token(&self, token: &str) -> Result<(), Error> {
    let result: Option<RefreshToken> = self.db
      .query(r#"
        UPDATE refresh_token 
        SET 
          is_valid = false,
          updated_at = time::now()
        WHERE token = $token
      "#)
      .bind(("token", token))
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
          is_valid = false,
          updated_at = time::now()
        WHERE user_id = $user_id
      "#)
      .bind(("user_id", user_id))
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
