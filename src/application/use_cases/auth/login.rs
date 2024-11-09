use crate::domain::error::Error;
use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::repositories::token_repository::TokenRepository;
use crate::domain::services::token::TokenService;
use crate::application::dtos::login::{login_request::LoginRequest, login_response::LoginResponse};

pub struct LoginUseCase<U, T>
where
  U: UserRepository,
  T: TokenRepository,
{
  user_repository: U,
  token_repository: T,
  token_service: TokenService,
}

impl<U, T> LoginUseCase<U, T>
where
  U: UserRepository,
  T: TokenRepository,
{
  pub fn new(user_repository: U, token_repository: T, token_service: TokenService) -> Self {
    Self { user_repository, token_repository, token_service }
  }

  pub async fn execute(&self, request: LoginRequest) -> Result<LoginResponse, Error> {
    let user = self.user_repository
      .authenticate(&request.email, &request.password)
      .await?
      .ok_or(Error::InvalidCredentials)?;
    
    // Generar tokens
    let token_pair = self.token_service.generate_token_pair(&user)?;

    // Guardar refresh token
    self.token_repository
      .create_refresh_token(&token_pair.refresh_token)
      .await?;

    Ok(LoginResponse {
      user_id: user.surreal_id.id().to_string(),
      email: user.email,
      access_token: token_pair.access_token,
      refresh_token: token_pair.refresh_token.token,
      access_token_expires_at: token_pair.refresh_token.expires_at,
    })
  }
}


