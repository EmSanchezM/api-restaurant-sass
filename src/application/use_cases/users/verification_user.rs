use std::str::FromStr;
use surrealdb::sql::Thing;

use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::error::Error;
use crate::domain::services::token::TokenService;

use crate::application::dtos::users::verification_status_user_request::VerificationStatusUserRequest;

pub struct VerificationUserUseCase<U> where U: UserRepository, 
{
  user_repository: U,
  token_service: TokenService,
}

impl<U> VerificationUserUseCase<U> where 
  U: UserRepository,
{
  pub fn new(user_repository: U, token_service: TokenService) -> Self {
    Self { user_repository, token_service }
  }

  pub async fn execute(&self, token: &str, request: &VerificationStatusUserRequest) -> Result<(), Error> {
    let claims = self.token_service.verify_access_token(token)?;

    if self.token_service.is_token_expired(&claims) {
      return Err(Error::TokenExpired);
    }

    let user_id = Thing::from_str(claims.sub.as_str()).unwrap();

    self.user_repository.set_verification_status(user_id.to_string(), request.is_verified).await?;
    
    Ok(())
  }
}