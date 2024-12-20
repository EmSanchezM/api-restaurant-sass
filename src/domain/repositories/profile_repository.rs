use async_trait::async_trait;

use crate::domain::entities::profile::Profile;
use crate::domain::error::Error;

#[async_trait]
pub trait ProfileRepository {
  async fn create(&self, profile: &Profile) -> Result<Profile, Error>;
  async fn update(&self, id: String, profile: &Profile) -> Result<Profile, Error>;
  async fn delete(&self, id: String) -> Result<(), Error>;
  async fn find_by_id(&self, id: String) -> Result<Option<Profile>, Error>;
  async fn find_by_user_id(&self, user_id: String) -> Result<Option<Profile>, Error>;
}
