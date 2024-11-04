use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::sql::Id as SurrealId;

use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::error::Error;

pub struct SurrealProfileRepository {
  db: Surreal<Ws>,
}

impl SurrealProfileRepository {
  pub fn new(db: Surreal<Ws>) -> Self {
    Self { db }
  }
}

#[async_trait]
impl ProfileRepository for SurrealProfileRepository {
  async fn create(&self, profile: &Profile) -> Result<Profile, Error> {
    let result: Option<Profile> = self.db
      .query(r#"
        CREATE profile SET
          user_id = $user_id,
          first_name = $first_name,
          last_name = $last_name,
          phone = $phone,
          address = $address,
          created_at = time::now()
      "#)
      .bind(("user_id", &profile.user_id))
      .bind(("first_name", &profile.first_name))
      .bind(("last_name", &profile.last_name))
      .bind(("phone", &profile.phone))
      .bind(("address", &profile.address))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn update(&self, profile: &Profile) -> Result<Profile, Error> {
    let result: Option<Profile> = self.db
      .query(r#"
        UPDATE profile 
        SET
          first_name = $first_name,
          last_name = $last_name,
          phone = $phone,
          address = $address,
          updated_at = time::now()
        WHERE id = $id
      "#)
      .bind(("id", &profile.id))
      .bind(("first_name", &profile.first_name))
      .bind(("last_name", &profile.last_name))
      .bind(("phone", &profile.phone))
      .bind(("address", &profile.address))
      .await?
      .take(0)?;

    result.ok_or(Error::ProfileNotFound)
  }

  async fn delete(&self, id: &SurrealId) -> Result<(), Error> {
    let result: Option<Profile> = self.db
      .query(r#"
          UPDATE profile 
          SET 
            is_active = false,
            updated_at = time::now()
          WHERE id = $id
      "#)
      .bind(("id", id))
      .await?
      .take(0)?;
    
    match result {
      Some(_) => Ok(()),
      None => Err(Error::ProfileNotFound)
    }
  }

  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Profile>, Error> {
    let profile: Option<Profile> = self.db
      .query("SELECT * FROM profile WHERE id = $id")
      .bind(("id", id))
      .await?
      .take(0)?;

    Ok(profile)
  }

  async fn find_by_user_id(&self, user_id: &SurrealId) -> Result<Option<Profile>, Error> {
    let profile: Option<Profile> = self.db
      .query("SELECT * FROM profile WHERE user_id = $user_id")
      .bind(("user_id", user_id))
      .await?
      .take(0)?;
    
    Ok(profile)
  }
}
