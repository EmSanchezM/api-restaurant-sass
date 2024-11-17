use std::str::FromStr;
use surrealdb::sql::Thing;

use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::error::Error;
use crate::domain::services::token::TokenService;

pub struct RemoveProfileUseCase<T> where T: ProfileRepository {
  profile_repository: T,
  token_service: TokenService,
}

impl<T> RemoveProfileUseCase<T> where T: ProfileRepository {
  pub fn new(profile_repository: T, token_service: TokenService) -> Self {
    Self { profile_repository, token_service }
  }

  pub async fn execute(&self, token: &str) -> Result<(), Error> {
    let claims = self.token_service.verify_access_token(token)?;

    if self.token_service.is_token_expired(&claims) {
      return Err(Error::TokenExpired);
    }

    let user_id = Thing::from_str(claims.sub.as_str()).unwrap();

    let profile = self.profile_repository.find_by_user_id(user_id.to_string()).await?;

    match profile {
      None => return Err(Error::ProfileNotFound),
      Some(profile) => {
        self.profile_repository.delete(profile.id.clone().unwrap().id.to_string()).await?;
        Ok(())
      }
    }
  }
}