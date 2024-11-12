use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::application::dtos::profile::create_profile_request::CreateProfileRequest;
use crate::application::dtos::profile::profile_response::ProfileResponse;
use crate::domain::error::Error;

use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::services::token::TokenService;

pub struct CreateProfileUseCase<T> where T: ProfileRepository {
  profile_repository: T,
  token_service: TokenService,
}

impl<T> CreateProfileUseCase<T> where T: ProfileRepository {
  pub fn new(profile_repository: T, token_service: TokenService) -> Self {
    Self { profile_repository, token_service }
  }

  pub async fn execute(&self, token: &str, request: &CreateProfileRequest) -> Result<ProfileResponse, Error> {
    let claims = self.token_service.verify_access_token(token)?;

    if self.token_service.is_token_expired(&claims) {
      return Err(Error::TokenExpired);
    }

    let user_id = SurrealId::new("user", claims.sub.as_str());

    let new_profile = Profile::new(
      user_id,
      request.first_name.clone(),
      request.last_name.clone(),
      request.phone.clone(),
      request.address.clone(),
      request.position.clone(),
      request.avatar.clone(),
      request.emergency_contact.clone(),
      request.birth_date.clone(),
    );
    
    let profile = self.profile_repository.create(&new_profile).await?;

    Ok(ProfileResponse { 
      id: profile.surreal_id.id().to_string(),
      user_id: profile.user_id.id().to_string(),
      first_name: profile.first_name,
      last_name: profile.last_name,
      phone: profile.phone,
      position: profile.position,
      birth_date: profile.birth_date,
      avatar: profile.avatar,
      address: Some(profile.address),
      emergency_contact: profile.emergency_contact,
      is_active: profile.is_active,
      created_at: profile.created_at
    })
  }
}