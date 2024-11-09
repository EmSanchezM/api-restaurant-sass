use serde::{Deserialize, Serialize};

use crate::domain::value_objects::user_types::UserType;

#[derive(Debug, Serialize, Deserialize)]  
pub struct RegisterRequest {
  pub email: String,
  pub password: String,
  pub user_type: UserType,
  pub created_by: Option<String>,
}