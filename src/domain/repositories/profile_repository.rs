use async_trait::async_trait;

use crate::domain::entities::profile::Profile;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;

#[async_trait]
pub trait ProfileRepository {
  async fn create(&self, profile: &Profile) -> Result<Profile, Error>;
  async fn update(&self, id: &SurrealId, profile: &Profile) -> Result<Profile, Error>;
  async fn delete(&self, id: &SurrealId) -> Result<(), Error>;
  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Profile>, Error>;
  async fn find_by_user_id(&self, user_id: &SurrealId) -> Result<Option<Profile>, Error>;
}
