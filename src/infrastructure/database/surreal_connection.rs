use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Client, Ws};
use std::sync::Arc;

use crate::domain::error::Error;

pub struct DatabaseConnection {
  pub client: Arc<Surreal<Client>>,
}

impl DatabaseConnection {
  pub async fn new(url: &str, namespace: &str, database: &str) -> Result<Self, Error> {
    let client = Surreal::new::<Ws>(url).await?;

    client.use_ns(namespace).use_db(database).await?;

    Ok(Self {
      client: Arc::new(client),
    })
  }

  pub fn get_client(&self) -> Arc<Surreal<Client>> {
    self.client.clone()
  }
}