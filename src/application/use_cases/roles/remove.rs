use crate::domain::repositories::role_repository::RoleRepository;
use crate::domain::error::Error;

pub struct RemoveRoleUseCase<T> where T: RoleRepository {
  role_repository: T,
}

impl<T> RemoveRoleUseCase<T> where T: RoleRepository {
  pub fn new(role_repository: T) -> Self {
    Self { role_repository }
  }

  pub async fn execute(&self, id: &str) -> Result<(), Error> {
    let role = self.role_repository.find_by_id(id.to_string()).await?;

    if role.is_none() {
      return Err(Error::RoleNotFound);
    }
    
    let role = role.unwrap();

    self.role_repository.delete(role.id.clone().unwrap().id.to_string()).await?;
    Ok(())
  }
}