use crate::domain::repositories::role_repository::RoleRepository;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;

pub struct RemoveRoleUseCase<T> where T: RoleRepository {
  role_repository: T,
}

impl<T> RemoveRoleUseCase<T> where T: RoleRepository {
  pub fn new(role_repository: T) -> Self {
    Self { role_repository }
  }

  pub async fn execute(&self, id: &str) -> Result<(), Error> {
    let role_id = SurrealId::new("role", id);
    let role = self.role_repository.find_by_id(&role_id).await?;

    if role.is_none() {
      return Err(Error::RoleNotFound);
    }
    
    let role = role.unwrap();

    self.role_repository.delete(&role.surreal_id).await?;
    Ok(())
  }
}