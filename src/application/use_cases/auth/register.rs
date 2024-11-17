use std::str::FromStr;
use surrealdb::sql::Thing;

use crate::domain::repositories::{
  user_repository::UserRepository,
  token_repository::TokenRepository,
};
use crate::domain::entities::user::User;
use crate::domain::error::Error;
use crate::domain::value_objects::{user_types::UserType, user_status::UserStatus};
use crate::application::dtos::register::{register_request::RegisterRequest, register_response::RegisterResponse, register_response::ProfileStatus};
use crate::domain::services::token::TokenService;

pub struct RegisterUseCase<U, T>
where
  U: UserRepository,
  T: TokenRepository,
{
  user_repository: U,
  token_repository: T,
  token_service: TokenService,
}

impl<U, T> RegisterUseCase<U, T>
where
  U: UserRepository,
  T: TokenRepository,
{
  pub fn new(user_repository: U, token_repository: T, token_service: TokenService) -> Self {
    Self {
      user_repository,
      token_repository,
      token_service,
    }
  }

  pub async fn execute(
    &self,
    request: RegisterRequest,
  ) -> Result<RegisterResponse, Error> {
    let user_type = request.user_type.clone();

    if let Some(_) = self.user_repository.find_by_email(&request.email).await? {
      tokio::time::sleep(std::time::Duration::from_millis(500)).await;
      return Err(Error::RegistrationFailed);
    }

    // Validar permisos según el tipo de usuario
    match user_type {
      UserType::Employee | UserType::Admin | UserType::SuperAdmin => {
        let creator_id = request.created_by
          .ok_or(Error::UnauthorizedOperation)?;
        
        self.validate_admin_permissions(&creator_id).await?;
      }
      UserType::Customer => {
        if request.created_by.is_some() {
          return Err(Error::InvalidOperation);
        }
      },
    }

    // Create user
    let new_user = User::new(
      request.email.clone(),
      request.password.clone(),
      UserStatus::PendingVerification,
      user_type,
    );

    let user = match self.user_repository
      .create(&new_user)
      .await {
        Ok(user) => user,
        Err(_) => return Err(Error::RegistrationFailed)
      };

    let user_with_roles_and_permissions = self.user_repository.find_by_id(user.id.clone().unwrap().id.to_string()).await?.unwrap();
    let roles = user_with_roles_and_permissions.roles.as_ref().unwrap().clone();
    
    // Assign roles to the user
    self.user_repository.assign_roles(user.id.clone().unwrap().id.to_string(), roles).await?;

    // Create refresh token
    let new_refresh_token = self.token_service.generate_refresh_token(&user_with_roles_and_permissions)?;

    let refresh_token = match self.token_repository.create_refresh_token(&new_refresh_token).await {
      Ok(token) => token,
      Err(_) => {
        return Err(Error::RegistrationFailed);
      }
    };

    let profile_status = match request.user_type {
      UserType::Customer => ProfileStatus::PendingCompletion,
      UserType::Employee | UserType::Admin | UserType::SuperAdmin => ProfileStatus::PendingApproval,
    };

    Ok(RegisterResponse {
      user,
      refresh_token,
      profile_status,
    })
  }

  async fn validate_admin_permissions(&self, creator_id: &str) -> Result<(), Error> {
    let creator_id = Thing::from_str(creator_id).unwrap();
    
    let creator = self.user_repository
      .find_by_id(creator_id.to_string())
      .await?
      .ok_or(Error::UnauthorizedOperation)?;

    if !creator.is_active {
      return Err(Error::UnauthorizedOperation);
    }

    if creator.user_type != UserType::Admin && creator.user_type != UserType::SuperAdmin {
      return Err(Error::UnauthorizedOperation);
    }

    Ok(())
  }
}


