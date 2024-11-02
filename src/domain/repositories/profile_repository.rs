use async_trait::async_trait;

use crate::domain::entities::profile::Profile;

#[async_trait]
pub trait ProfileRepository {
  async fn create(&self, profile: &Profile) -> Result<Profile, Error>;
  async fn update(&self, profile: &Profile) -> Result<Profile, Error>;
  async fn delete(&self, id: &SurrealId) -> Result<(), Error>;
  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Profile>, Error>;
  async fn find_by_user_id(&self, user_id: &SurrealId) -> Result<Option<Profile>, Error>;
}
