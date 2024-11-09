use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdatePermissionRequest {
  pub name: Option<String>,
  pub description: Option<String>,
  pub resource: Option<String>,
  pub action: Option<String>,
}
