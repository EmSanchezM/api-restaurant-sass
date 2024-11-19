use actix_web::{post, get, put, delete, web, HttpResponse};

use crate::application::dtos::profile::{
  create_profile_request::CreateProfileRequest,
  update_profile_request::UpdateProfileRequest,
};

use crate::application::use_cases::profile::{
  create::CreateProfileUseCase,
  update::UpdateProfileUseCase,
  get_by_id::GetProfileByIdUseCase,
  get_by_user::GetProfileByUserUseCase,
  remove::RemoveProfileUseCase
};

use crate::infrastructure::repositories::surreal_profile_repository::SurrealProfileRepository;
use crate::infrastructure::database::surreal_connection::DatabaseConnection;
use crate::domain::services::token::TokenService;
use crate::infrastructure::config_env::Config;

#[post("/")]
pub async fn create_profile_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
  request: web::Json<CreateProfileRequest>
) -> HttpResponse {
  
  let repo = SurrealProfileRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());
  let token = "token";

  match CreateProfileUseCase::new(repo, token_service)
    .execute(token, &request.into_inner()).await {
      Ok(response) => HttpResponse::Ok().json(response),
      Err(_) => {
        //Todo: Log de errores
        //error!("Error en el registro: {:?}", err);
        HttpResponse::InternalServerError().body("Error al crear perfil")
      }
    }
}

#[put("/")]
pub async fn update_profile_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
  request: web::Json<UpdateProfileRequest>,
) -> HttpResponse {

  let repo = SurrealProfileRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());
  let token = "token";

  match UpdateProfileUseCase::new(repo, token_service)
    .execute(token,&request.into_inner()).await {
      Ok(response) => HttpResponse::Ok().json(response),
      Err(_) => HttpResponse::InternalServerError().body("Error al actualizar perfil")
    }
}

#[delete("/")]
pub async fn delete_profile_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
) -> HttpResponse {

  let repo = SurrealProfileRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());
  let token = "token";

  match RemoveProfileUseCase::new(repo, token_service).execute(token).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().body("Error al eliminar perfil")
  }
}

#[get("/")]
pub async fn get_profile_handler(
  db_connection: web::Data<DatabaseConnection>,
  config: web::Data<Config>,
) -> HttpResponse {
  let repo = SurrealProfileRepository::new(&db_connection);
  let token_service = TokenService::new(config.token_config.clone());
  let token = "token";
  
  match GetProfileByUserUseCase::new(repo, token_service).execute(token).await {
    Ok(profile) => HttpResponse::Ok().json(profile),
    Err(_) => HttpResponse::InternalServerError().body("Error al obtener perfil")
  }
}

#[get("/{id}")]
pub async fn get_profile_by_id_handler(
  db_connection: web::Data<DatabaseConnection>,
  id: web::Path<String>
) -> HttpResponse {
  let repo = SurrealProfileRepository::new(&db_connection);

  match GetProfileByIdUseCase::new(repo).execute(&id.into_inner()).await {
    Ok(profile) => HttpResponse::Ok().json(profile),
    Err(_) => HttpResponse::InternalServerError().body("Error al obtener perfil")
  }
}