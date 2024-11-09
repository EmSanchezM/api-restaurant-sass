use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
  
#[derive(Debug, Serialize, Deserialize)]
pub struct RoleResponse {
  pub id: String,
  pub name: String,
  pub description: String,
  pub hierarchy_level: i32,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
}
