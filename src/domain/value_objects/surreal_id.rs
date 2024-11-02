use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurrealId {
  tb: String,
  id: String,
}

impl SurrealId {
  pub fn new(tb: &str, id: &str) -> Self {
    Self {
      tb: tb.to_string(),
      id: id.to_string(),
    }
  }

  pub fn table(&self) -> &str {
    &self.tb
  }

  pub fn id(&self) -> &str {
    &self.id
  }
}

impl fmt::Display for SurrealId {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}:{}", self.tb, self.id)
  }
}
