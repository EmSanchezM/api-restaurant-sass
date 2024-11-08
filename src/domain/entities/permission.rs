use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

use crate::domain::value_objects::surreal_id::SurrealId;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
  #[serde(rename = "id")]
  pub surreal_id: SurrealId,
  pub name: String,
  pub description: String,
  pub resource: Resource,
  pub action: Action,
  pub created_at: DateTime<Utc>,
}

impl Permission {
  pub fn new(name: String, description: String, resource: Resource, action: Action) -> Self {
    Self { surreal_id: SurrealId::generate("permission"), name, description, resource, action, created_at: Utc::now() }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Resource {
  Orders,
  Inventory,
  Users,
  Employees,
  Reports,
  Settings,
  Menu,
  Transactions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Action {
  Create,
  Read,
  Update,
  Delete,
  Approve,
  Cancel,
  Manage,
}
