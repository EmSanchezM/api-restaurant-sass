use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::domain::entities::role::Role;
use crate::domain::entities::permission::Permission;
use crate::domain::value_objects::{user_status::UserStatus, user_types::UserType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
  pub id: Option<Thing>,
  pub email: String,
  pub password: String,
  pub status: UserStatus,
  pub user_type: UserType,
  pub roles: Option<Vec<Role>>,
  pub permissions: Option<Vec<Permission>>,
  pub is_verified: bool,
  pub is_active: bool,
  pub failed_login_attempts: i32,
  pub last_login: Option<DateTime<Utc>>,
  pub locked_until: Option<DateTime<Utc>>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

impl User {
  pub fn new(email: String, password: String, status: UserStatus, user_type: UserType) -> Self {
    Self { 
      id: None,
      email, 
      password, 
      status, 
      user_type, 
      is_verified: false, 
      is_active: true, 
      failed_login_attempts: 0, 
      last_login: None, 
      locked_until: None,
      roles: None,
      permissions: None,
      created_at: Utc::now(), 
      updated_at: Utc::now() 
    }
  }
}