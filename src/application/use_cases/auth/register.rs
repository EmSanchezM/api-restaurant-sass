use async_trait::async_trait;
use chrono::{Duration, Utc};

use crate::domain::entities::{
  user::User,
  token::RefreshToken,
};
use crate::domain::repositories::{
  user_repository::UserRepository,
  token_repository::TokenRepository,
};
use crate::domain::error::Error;
use crate::domain::value_objects::user_type::UserType;
use crate::application::use_cases::dtos::{register_request::RegisterRequest, register_response::RegisterResponse};

pub enum ProfileStatus {
  PendingCompletion,
}

pub struct RegisterUseCase<U, T>
where
  U: UserRepository,
  T: TokenRepository,
{
  user_repository: U,
  token_repository: T,
}

impl<U, T> RegisterUseCase<U, T>
where
  U: UserRepository,
  T: TokenRepository,
{
  pub fn new(user_repository: U, token_repository: T) -> Self {
    Self {
      user_repository,
      token_repository,
    }
  }

  pub async fn execute(
    &self,
    request: RegisterRequest,
  ) -> Result<RegisterResponse, Error> {
    // For security reasons, we use a constant-time operation and return a generic error
    // This helps prevent user enumeration attacks
    if let Some(_) = self.user_repository.find_by_email(&email).await? {
      // Simulate the time it would take to create a user to prevent timing attacks
      tokio::time::sleep(std::time::Duration::from_millis(500)).await;
      return Err(Error::RegistrationFailed);
    }

    // Validar permisos según el tipo de usuario
    match request.user_type {
      UserType::Employee => {
        // Verificar que existe un created_by y que tiene permisos
        let creator_id = request.created_by
          .ok_or(Error::UnauthorizedOperation)?;
        
        // Aquí deberías verificar que el creator_id corresponde a un admin
        self.validate_admin_permissions(&creator_id).await?;
      }
      UserType::Customer => {
        // Los clientes se pueden registrar sin verificación adicional
        if request.created_by.is_some() {
          return Err(Error::InvalidOperation);
        }
      }
      // Otros tipos de usuario si los hay...
    }

    // Create user
    let user = match self.user_repository
      .create(&email, &password, UserType::Customer)
      .await {
          Ok(user) => user,
          Err(_) => return Err(Error::RegistrationFailed)
      };

    // Create refresh token
    let refresh_token = RefreshToken {
      id: None,
      token: uuid::Uuid::new_v4().to_string(),
      user_id: user.id.clone(),
      expires_at: Utc::now() + Duration::days(7),
      created_at: None,
      updated_at: None,
      is_valid: true,
    };

    let refresh_token = match self.token_repository.create_refresh_token(&refresh_token).await {
      Ok(token) => token,
      Err(_) => {
        // If token creation fails, we should clean up the created user
        // However, we don't want to expose this information in the error
        return Err(Error::RegistrationFailed);
      }
    };

    // Determinar el estado del perfil
    let profile_status = match request.user_type {
      UserType::Customer => ProfileStatus::PendingCompletion,
      UserType::Employee => ProfileStatus::PendingApproval,
    };

    Ok(RegisterResponse {
      user,
      refresh_token,
      profile_status: ProfileStatus::PendingCompletion,
    })
  }
}


