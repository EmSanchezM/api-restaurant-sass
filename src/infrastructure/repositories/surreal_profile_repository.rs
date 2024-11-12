use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;

use crate::infrastructure::database::surreal_connection::DatabaseConnection;
use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;
pub struct SurrealProfileRepository {
  db: Arc<Surreal<Client>>,
}

impl SurrealProfileRepository {
  pub fn new(connection: &DatabaseConnection) -> Self {
    Self { 
      db: connection.get_client()
    }
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
          birth_date = $birth_date,
          avatar = $avatar,
          emergency_contact = $emergency_contact,
          created_at = time::now()
      "#)
      .bind(("user_id", &profile.user_id))
      .bind(("first_name", &profile.first_name))
      .bind(("last_name", &profile.last_name))
      .bind(("phone", &profile.phone))
      .bind(("address", &profile.address))
      .bind(("birth_date", &profile.birth_date))
      .bind(("avatar", &profile.avatar))
      .bind(("emergency_contact", &profile.emergency_contact))
      .await?
      .take(0)?;

    result.ok_or(Error::CreationFailed)
  }

  async fn update(&self, id: &SurrealId, profile: &Profile) -> Result<Profile, Error> {
    let result: Option<Profile> = self.db
      .query(r#"
        UPDATE type::thing("profile", $id) 
        SET
          first_name = $first_name,
          last_name = $last_name,
          phone = $phone,
          address = $address,
          birth_date = $birth_date,
          avatar = $avatar,
          emergency_contact = $emergency_contact,
          updated_at = time::now()
      "#)
      .bind(("id", id.id()))
      .bind(("first_name", &profile.first_name))
      .bind(("last_name", &profile.last_name))
      .bind(("phone", &profile.phone))
      .bind(("address", &profile.address))
      .bind(("birth_date", &profile.birth_date))
      .bind(("avatar", &profile.avatar))
      .bind(("emergency_contact", &profile.emergency_contact))
      .await?
      .take(0)?;

    result.ok_or(Error::ProfileNotFound)
  }

  async fn delete(&self, id: &SurrealId) -> Result<(), Error> {
    let result: Option<Profile> = self.db
      .query(r#"
        UPDATE type::thing("profile", $id) 
        SET
          is_active = false,
          updated_at = time::now()
      "#)
      .bind(("id", id.id()))
      .await?
      .take(0)?;
    
    match result {
      Some(_) => Ok(()),
      None => Err(Error::ProfileNotFound)
    }
  }

  async fn find_by_id(&self, id: &SurrealId) -> Result<Option<Profile>, Error> {
    let profile: Option<Profile> = self.db
      .query(r#"SELECT * FROM profile type::thing("profile", $id)"#)
      .bind(("id", id.id()))
      .await?
      .take(0)?;

    Ok(profile)
  }

  async fn find_by_user_id(&self, user_id: &SurrealId) -> Result<Option<Profile>, Error> {
    let profile: Option<Profile> = self.db
      .query(r#"
        SELECT * FROM profile WHERE user_id = type::thing("profile", $id)
      "#)
      .bind(("id", user_id.id()))
      .await?
      .take(0)?;
    
    Ok(profile)
  }
}
