use crate::domain::entities::profile::Profile;
use crate::domain::value_objects::surreal_id::SurrealId;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::error::Error;

use crate::application::dtos::profile::update_profile_request::UpdateProfileRequest;
use crate::application::dtos::profile::profile_response::ProfileResponse;

use crate::domain::services::token::TokenService;

pub struct UpdateProfileUseCase<T> where T: ProfileRepository {
  profile_repository: T,
  token_service: TokenService,
}

impl<T> UpdateProfileUseCase<T> where T: ProfileRepository {
  pub fn new(profile_repository: T, token_service: TokenService) -> Self {
    Self { profile_repository, token_service }
  }

  pub async fn execute(&self, token: &str, request: &UpdateProfileRequest) -> Result<ProfileResponse, Error> {

    let claims = self.token_service.verify_access_token(token)?;

    if self.token_service.is_token_expired(&claims) {
      return Err(Error::TokenExpired);
    }

    let user_id = SurrealId::new("user", claims.sub.as_str());

    let profile = self.profile_repository.find_by_user_id(&user_id).await?;

        match profile {
          None => return Err(Error::ProfileNotFound),
          Some(profile) => {
          
            let payload = Profile::new(
              profile.user_id,
              request.first_name.clone().unwrap_or(profile.first_name.clone()),
              request.last_name.clone().unwrap_or(profile.last_name.clone()),
              request.phone.clone().unwrap_or(profile.phone.clone()),
              request.address.clone().unwrap_or(profile.address.clone()),
          request.position.clone(),
          request.avatar.clone(),
          request.emergency_contact.clone(),
          request.birth_date.clone().unwrap_or(profile.birth_date.clone()),
        );
        
        let updated_profile = self.profile_repository.update(&profile.surreal_id, &payload).await?;
        
        Ok(ProfileResponse {
          id: updated_profile.surreal_id.id().to_string(),
          user_id: updated_profile.user_id.id().to_string(),
          first_name: updated_profile.first_name,
          last_name: updated_profile.last_name,
          phone: updated_profile.phone,
          position: updated_profile.position,
          birth_date: updated_profile.birth_date,
          avatar: updated_profile.avatar,
          address: Some(updated_profile.address),
          emergency_contact: updated_profile.emergency_contact,
          is_active: updated_profile.is_active,
          created_at: updated_profile.created_at,
        })
      }
    }
  }
}
