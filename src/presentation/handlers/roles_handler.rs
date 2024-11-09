use actix_web::{post, get, put, delete, web, HttpResponse};

use crate::application::dtos::roles::{
  create_role_request::CreateRoleRequest,
  update_role_request::UpdateRoleRequest,
}; 

use crate::application::use_cases::roles::{
  create::CreateRoleUseCase,
  update::UpdateRoleUseCase,
  get_all::GetAllRolesUseCase,
  get_by_id::GetRoleByIdUseCase,
  remove::RemoveRoleUseCase
};

use crate::infrastructure::repositories::surreal_role_repository::SurrealRoleRepository;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;

#[post("/roles")]
pub async fn create_role_handler(
  db_connection: web::Data<DatabaseConnection>,
  request: web::Json<CreateRoleRequest>
) -> HttpResponse {
  
  let repo = SurrealRoleRepository::new(&db_connection);
  
  match CreateRoleUseCase::new(repo)
    .execute(&request.into_inner()).await {
      Ok(response) => HttpResponse::Ok().json(response),
      Err(_) => {
        //Todo: Log de errores
        //error!("Error en el registro: {:?}", err);
        HttpResponse::InternalServerError().body("Error al crear rol")
      }
    }
}

#[put("/roles/{id}")]
pub async fn update_role_handler(
  db_connection: web::Data<DatabaseConnection>,
  request: web::Json<UpdateRoleRequest>,
  id: web::Path<String>
) -> HttpResponse {

  let repo = SurrealRoleRepository::new(&db_connection);

  match UpdateRoleUseCase::new(repo)
    .execute(&id.into_inner(),&request.into_inner()).await {
      Ok(response) => HttpResponse::Ok().json(response),
      Err(_) => HttpResponse::InternalServerError().body("Error al actualizar rol")
    }
}

#[delete("/roles/{id}")]
pub async fn delete_role_handler(
  db_connection: web::Data<DatabaseConnection>,
  id: web::Path<String>
) -> HttpResponse {

  let repo = SurrealRoleRepository::new(&db_connection);

  match RemoveRoleUseCase::new(repo).execute(&id.into_inner()).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().body("Error al eliminar rol")
  }
}

#[get("/roles")]
pub async fn get_all_roles_handler(
  db_connection: web::Data<DatabaseConnection>
) -> HttpResponse {
  let repo = SurrealRoleRepository::new(&db_connection);

  match GetAllRolesUseCase::new(repo).execute().await {
    Ok(roles) => HttpResponse::Ok().json(roles),
    Err(_) => HttpResponse::InternalServerError().body("Error al obtener roles")
  }
}

#[get("/roles/{id}")]
pub async fn get_role_by_id_handler(
  db_connection: web::Data<DatabaseConnection>,
  id: web::Path<String>
) -> HttpResponse {
  let repo = SurrealRoleRepository::new(&db_connection);

  match GetRoleByIdUseCase::new(repo).execute(&id.into_inner()).await {
    Ok(role) => HttpResponse::Ok().json(role),
    Err(_) => HttpResponse::InternalServerError().body("Error al obtener rol")
  }
}