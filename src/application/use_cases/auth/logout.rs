use crate::domain::repositories::token_repository::TokenRepository;
use crate::domain::error::Error;
use crate::domain::services::token::TokenService;
use crate::domain::value_objects::surreal_id::SurrealId;

pub struct LogoutUseCase<T> where T: TokenRepository {
  token_repository: T,
  token_service: TokenService,
}

impl<T> LogoutUseCase<T> where T: TokenRepository {
  pub fn new(token_repository: T, token_service: TokenService) -> Self {
    Self { token_repository, token_service }
  }

  pub async fn execute(&self, token: &str) -> Result<(), Error> {
    let claims = self.token_service.verify_access_token(token)?;

    if self.token_service.is_token_expired(&claims) {
      return Err(Error::TokenExpired);
    }

    let user_id = SurrealId::new("user", claims.sub.as_str());

    self.token_repository.invalidate_refresh_token(&user_id).await?;
    Ok(())
  }
}