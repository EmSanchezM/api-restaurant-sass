use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePermissionRequest {
  pub name: String, 
  pub description: String,
  pub resource: String,
  pub action: String,
}
