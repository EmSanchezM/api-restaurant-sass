use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::application::dtos::profile::profile_response::ProfileResponse;
use crate::domain::error::Error;

pub struct GetProfileByIdUseCase<T> where T: ProfileRepository {
  profile_repository: T,
}

impl<T> GetProfileByIdUseCase<T> where T: ProfileRepository {
  pub fn new(profile_repository: T) -> Self {
    Self { profile_repository }
  }

  pub async fn execute(&self, id: &str) -> Result<ProfileResponse, Error> {
    let profile = self.profile_repository.find_by_id(id.to_string()).await?;
    
    match profile {
      None => Err(Error::ProfileNotFound),
      Some(profile) => Ok(ProfileResponse {
        id: profile.id.clone().unwrap().id.to_string(),
        user_id: profile.user_id.to_string(),
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
}