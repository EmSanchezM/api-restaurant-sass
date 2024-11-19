use std::str::FromStr;
use surrealdb::sql::Thing;

use crate::domain::repositories::user_repository::UserRepository;
use crate::domain::error::Error;
use crate::domain::services::token::TokenService;

use crate::application::dtos::profile::profile_response::ProfileResponse;
use crate::application::dtos::users::user_response::UserResponse;

use crate::domain::value_objects::user_types::UserType;

pub struct GetAllUsersUseCase<U> where U: UserRepository, 
{
  user_repository: U,
  token_service: TokenService,
}

impl<U> GetAllUsersUseCase<U> where 
  U: UserRepository,
{
  pub fn new(user_repository: U, token_service: TokenService) -> Self {
    Self { user_repository, token_service }
  }

  pub async fn execute(&self, token: &str) -> Result<Vec<UserResponse>, Error> {
    let claims = self.token_service.verify_access_token(token)?;

    if self.token_service.is_token_expired(&claims) {
      return Err(Error::TokenExpired);
    }

    let user_id = Thing::from_str(claims.sub.as_str()).unwrap();

    let current_user = self.user_repository.find_by_id(user_id.to_string()).await?.unwrap();
    
    if current_user.user_type != UserType::SuperAdmin {
      return Err(Error::UnauthorizedAccess);
    }

    let users = self.user_repository.find_all().await?;
  
    Ok(users.iter().map(|user| UserResponse {
      id: user.id.clone().unwrap().id.to_string(),
      user_type: user.user_type.clone(),
      status: user.status.clone(),
      failed_login_attempts: user.failed_login_attempts,
      last_login: user.last_login.map(|dt| dt.to_string()),
      locked_until: user.locked_until.map(|dt| dt.to_string()),
      profile: match &user.profile {
        None => None,
        Some(profile) => Some(ProfileResponse {
          id: profile.id.clone().unwrap().id.to_string(),
          user_id: profile.user_id.to_string(),
          first_name: profile.first_name.clone(),
          last_name: profile.last_name.clone(),
          phone: profile.phone.clone(),
          address: Some(profile.address.clone()),
          position: profile.position.clone(),
          birth_date: profile.birth_date.clone(),
          avatar: profile.avatar.clone(),
          emergency_contact: profile.emergency_contact.clone(),
          is_active: profile.is_active,
          created_at: user.profile.clone().unwrap().created_at
        })
      },
      email: user.email.clone(),
      is_active: user.is_active,
      created_at: user.created_at.to_string(),
    }).collect())
  }

}