use async_trait::async_trait;
use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use std::sync::Arc;

use crate::infrastructure::database::surreal_connection::DatabaseConnection;
use crate::domain::entities::profile::Profile;
use crate::domain::repositories::profile_repository::ProfileRepository;
use crate::domain::error::Error;

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
    let result: Option<Profile> = self.db.create("profiles").content(profile.clone()).await?;

    result.ok_or(Error::CreationFailed)
  }

  async fn update(&self, id: String, profile: &Profile) -> Result<Profile, Error> {
    let result: Option<Profile> = self.db.update(("profiles", id)).content(profile.clone()).await?;

    result.ok_or(Error::ProfileNotFound)
  }

  async fn delete(&self, id: String) -> Result<(), Error> {
    let result: Option<Profile> = self.db
      .query(r#"
        UPDATE type::thing("profiles", $id) 
        SET
          is_active = false,
          updated_at = time::now()
      "#)
      .bind(("id", id))
      .await?
      .take(0)?;
    
    match result {
      Some(_) => Ok(()),
      None => Err(Error::ProfileNotFound)
    }
  }

  async fn find_by_id(&self, id: String) -> Result<Option<Profile>, Error> {
    let profile: Option<Profile> = self.db
      .query(r#"SELECT * FROM profiles type::thing("profiles", $id)"#)
      .bind(("id", id.clone()))
      .await?
      .take(0)?;

    Ok(profile)
  }

  async fn find_by_user_id(&self, user_id: String) -> Result<Option<Profile>, Error> {
    let profile: Option<Profile> = self.db
      .query(r#"
        SELECT * FROM profiles WHERE user_id = type::thing("profiles", $id)
      "#)
      .bind(("id", user_id.clone()))
      .await?
      .take(0)?;
    
    Ok(profile)
  }
}
