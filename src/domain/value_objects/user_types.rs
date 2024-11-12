use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserType {
  Customer,
  Employee,
  Admin,
  SuperAdmin,
}

impl std::str::FromStr for UserType {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {  
      "customer" => UserType::Customer,
      "employee" => UserType::Employee,
      "admin" => UserType::Admin,
      "super_admin" => UserType::SuperAdmin,
      _ => return Err(format!("Invalid user type: {}", s)),
    })
  }
}