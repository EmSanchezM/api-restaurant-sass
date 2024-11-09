use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateRoleRequest {
  pub name: Option<String>,
  pub description: Option<String>,
  pub hierarchy_level: Option<i32>,
}
