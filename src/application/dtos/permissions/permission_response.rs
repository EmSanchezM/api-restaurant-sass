use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
  
#[derive(Debug, Serialize, Deserialize)]
pub struct PermissionResponse {
  pub id: String,
  pub name: String,
  pub description: String,
  pub resource: String,
  pub action: String,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
}
