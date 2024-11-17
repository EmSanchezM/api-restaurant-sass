use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
  pub id: Option<Thing>,
  pub name: String,
  pub description: String,
  pub resource: Resource,
  pub action: Action,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
}

impl Permission {
  pub fn new(name: String, description: String, resource: Resource, action: Action) -> Self {
    Self { id: None, name, description, resource, action, is_active: true, created_at: Utc::now() }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Resource {
  Orders,
  Inventory,
  Users,
  Employees,
  Reports,
  Settings,
  Menu,
  Transactions,
}

impl std::str::FromStr for Resource {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "orders" => Resource::Orders,
      "inventory" => Resource::Inventory,
      "users" => Resource::Users,
      "employees" => Resource::Employees,
      "reports" => Resource::Reports,
      "settings" => Resource::Settings,
      "menu" => Resource::Menu,
      "transactions" => Resource::Transactions,
      _ => return Err(format!("Invalid resource: {}", s)),
    })
  }
}

impl Resource {
  pub fn to_string(&self) -> String {
    match self {
      Resource::Orders => "orders".to_string(),
      Resource::Inventory => "inventory".to_string(),
      Resource::Users => "users".to_string(),
      Resource::Employees => "employees".to_string(),
      Resource::Reports => "reports".to_string(),
      Resource::Settings => "settings".to_string(),
      Resource::Menu => "menu".to_string(),
      Resource::Transactions => "transactions".to_string(),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Action {
  Create,
  Read,
  Update,
  Delete,
  Approve,
  Cancel,
  Manage,
}

impl std::str::FromStr for Action {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s {
      "create" => Action::Create,
      "read" => Action::Read,
      "update" => Action::Update,
      "delete" => Action::Delete,
      "approve" => Action::Approve,
      "cancel" => Action::Cancel,
      "manage" => Action::Manage,
      _ => return Err(format!("Invalid action: {}", s)),
    })
  }
}

impl Action {
  pub fn to_string(&self) -> String {
    match self {
      Action::Create => "create".to_string(),
      Action::Read => "read".to_string(),
      Action::Update => "update".to_string(),
      Action::Delete => "delete".to_string(),
      Action::Approve => "approve".to_string(),
      Action::Cancel => "cancel".to_string(),
      Action::Manage => "manage".to_string(),
    }
  }
}