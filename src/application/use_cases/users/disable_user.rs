use std::str::FromStr;
use surrealdb::sql::Thing;

use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::error::Error;
use crate::domain::services::token::TokenService;

pub struct DisableUserUseCase<U> where U: UserRepository, 
{
  user_repository: U,
  token_service: TokenService,
}

impl<U> DisableUserUseCase<U> where 
  U: UserRepository,
{
  pub fn new(user_repository: U, token_service: TokenService) -> Self {
    Self { user_repository, token_service }
  }

  pub async fn execute(&self, token: &str) -> Result<(), Error> {
    let claims = self.token_service.verify_access_token(token)?;

    if self.token_service.is_token_expired(&claims) {
      return Err(Error::TokenExpired);
    }

    let user_id = Thing::from_str(claims.sub.as_str()).unwrap();

    //TODO: Solo los super admins pueden deshabilitar a los usuarios

    self.user_repository.update_failed_login_attempts(user_id.to_string(), 0).await?;
    self.user_repository.delete(user_id.to_string()).await?;
    
    Ok(())
  }
}