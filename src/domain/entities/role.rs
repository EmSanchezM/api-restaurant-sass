use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::value_objects::surreal_id::SurrealId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
  #[serde(rename = "id")]
  pub surreal_id: SurrealId,
  pub name: String,
  pub description: String,
  pub hierarchy_level: i32,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
}

impl Role {
  pub fn new(name: String, description: String, hierarchy_level: i32) -> Self {
    Self { surreal_id: SurrealId::generate("role"), name, description, hierarchy_level, is_active: true, created_at: Utc::now() }
  }
}