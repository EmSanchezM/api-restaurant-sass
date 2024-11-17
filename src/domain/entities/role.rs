use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
  pub id: Option<Thing>,
  pub name: String,
  pub description: String,
  pub hierarchy_level: i32,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
}

impl Role {
  pub fn new(name: String, description: String, hierarchy_level: i32) -> Self {
    Self { id: None, name, description, hierarchy_level, is_active: true, created_at: Utc::now() }
  }
}