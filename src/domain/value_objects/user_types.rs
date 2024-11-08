use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserType {
  Customer,
  Employee,
  Admin,
  SuperAdmin,
}