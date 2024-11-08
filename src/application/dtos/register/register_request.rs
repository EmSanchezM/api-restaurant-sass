use crate::domain::value_objects::{surreal_id::SurrealId, user_types::UserType};

pub struct RegisterRequest {
  pub email: String,
  pub password: String,
  pub user_type: UserType,
  pub created_by: Option<String>,
}