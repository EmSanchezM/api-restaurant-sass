use crate::domain::repositories::permission_repository::PermissionRepository;
use crate::domain::error::Error;
use crate::domain::value_objects::surreal_id::SurrealId;

pub struct RemovePermissionUseCase<T> where T: PermissionRepository {
  permission_repository: T,
}

impl<T> RemovePermissionUseCase<T> where T: PermissionRepository {
  pub fn new(permission_repository: T) -> Self {
    Self { permission_repository }
  }

  pub async fn execute(&self, id: &str) -> Result<(), Error> {
    let permission_id = SurrealId::new("permission", id);
    let permission = self.permission_repository.find_by_id(&permission_id).await?;

    match permission {
      None => return Err(Error::PermissionNotFound),
      Some(permission) => {
        self.permission_repository.delete(&permission.surreal_id).await?;
        Ok(())
      }
    }
  }
}