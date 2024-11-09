use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateRoleRequest {
  pub name: String, 
  pub description: String,
  pub hierarchy_level: i32,
}
