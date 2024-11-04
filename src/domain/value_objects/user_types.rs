use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserType {
  Customer,
  Employee,
  Admin,
}