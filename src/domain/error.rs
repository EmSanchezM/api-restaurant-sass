use thiserror::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
  pub code: u16,
  pub message: String,
}

#[derive(Error, Debug)]
pub enum Error {
  // Errores de autenticación
  #[error("Invalid credentials")]
  InvalidCredentials,
  
  #[error("Token expired")]
  TokenExpired,
  
  #[error("Invalid token")]
  InvalidToken,
  
  #[error("Unauthorized access")]
  UnauthorizedAccess,

  // Errores de usuario
  #[error("User not found")]
  UserNotFound,

  #[error("User already exists with email: {0}")]
  UserAlreadyExists(String),
  
  #[error("Invalid email format")]
  InvalidEmail,

  #[error("Failed to create user: {0}")]
  UserCreationError(String),

  #[error("Failed to update user: {0}")]
  UserUpdateError(String),

  #[error("Failed to delete user: {0}")]
  UserDeletionError(String),

  #[error("Failed to fetch users: {0}")]
  UserFetchError(String),

  // Errores de registro y operaciones
  #[error("Registration failed")]
  RegistrationFailed,

  #[error("Unauthorized operation")]
  UnauthorizedOperation,

  // Errores de perfil
  #[error("Profile not found")]
  ProfileNotFound,
  
  #[error("Profile already exists for user")]
  ProfileAlreadyExists,
  
  #[error("Invalid phone number")]
  InvalidPhone,

  // Errores de rol y permisos
  #[error("Role not found")]
  RoleNotFound,
  
  #[error("Permission not found")]
  PermissionNotFound,
  
  #[error("Invalid permission")]
  InvalidPermission,

  // Errores de validación
  #[error("Validation error: {0}")]
  ValidationError(String),

  #[error("Invalid input: {0}")]
  InvalidInput(String),

  #[error("Invalid operation")]
  InvalidOperation,

  // Errores de base de datos
  #[error("Database error: {0}")]
  DatabaseError(String),
    
  #[error("Connection error: {0}")]
  ConnectionError(String),
  
  #[error("Transaction error: {0}")]
  TransactionError(String),

  // Errores de SurrealDB específicos
  #[error("SurrealDB error: {0}")]
  SurrealDBError(String),

  // Errores de serialización/deserialización
  #[error("Serialization error: {0}")]
  SerializationError(String),

  #[error("Deserialization error: {0}")]
  DeserializationError(String),

  // Errores de infraestructura
  #[error("Infrastructure error: {0}")]
  InfrastructureError(String),
  
  #[error("Service unavailable: {0}")]
  ServiceUnavailable(String),

  // Errores genéricos
  #[error("Internal server error")]
  InternalServerError,

  #[error("Unknown error occurred")]
  Unknown,

  #[error("Creation failed")]
  CreationFailed,

  #[error("Config error: {0}")]
  ConfigError(String),

  #[error("Token generation error: {0}")]
  TokenGenerationError(String),

  // Wrapper para errores externos
  #[error(transparent)]
  ValidationErrors(#[from] validator::ValidationErrors),
  
  #[error(transparent)]
  IOError(#[from] std::io::Error),
}

impl From<surrealdb::Error> for Error {
  fn from(err: surrealdb::Error) -> Self {
    match err {
      surrealdb::Error::Db(db_err) => Error::DatabaseError(db_err.to_string()),
      surrealdb::Error::Api(api_err) => Error::InfrastructureError(api_err.to_string()),
      _ => Error::SurrealDBError(err.to_string()),
    }
  }
}

impl actix_web::error::ResponseError for Error {
  fn error_response(&self) -> actix_web::HttpResponse {
    let error_response = ErrorResponse {
      code: self.status_code().as_u16(),
      message: self.to_string(),
    };

    actix_web::HttpResponse::build(self.status_code())
      .json(error_response)
  }

  fn status_code(&self) -> actix_web::http::StatusCode {
    use actix_web::http::StatusCode;

    match *self {
      Error::InvalidCredentials => StatusCode::UNAUTHORIZED,
      Error::TokenExpired => StatusCode::UNAUTHORIZED,
      Error::InvalidToken => StatusCode::UNAUTHORIZED,
      Error::UserCreationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::UserUpdateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::UserDeletionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::UserFetchError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::DeserializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::UnauthorizedAccess => StatusCode::FORBIDDEN,
      Error::UserNotFound => StatusCode::NOT_FOUND,
      Error::UserAlreadyExists(_) => StatusCode::CONFLICT,
      Error::ProfileNotFound => StatusCode::NOT_FOUND,
      Error::ProfileAlreadyExists => StatusCode::CONFLICT,
      Error::ValidationError(_) => StatusCode::BAD_REQUEST,
      Error::InvalidInput(_) => StatusCode::BAD_REQUEST,
      Error::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::ConnectionError(_) => StatusCode::SERVICE_UNAVAILABLE,
      Error::InvalidEmail => StatusCode::BAD_REQUEST,
      Error::InvalidPhone => StatusCode::BAD_REQUEST,
      Error::RoleNotFound => StatusCode::NOT_FOUND,
      Error::PermissionNotFound => StatusCode::NOT_FOUND,
      Error::InvalidPermission => StatusCode::FORBIDDEN,
      Error::TransactionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::InfrastructureError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::ServiceUnavailable(_) => StatusCode::SERVICE_UNAVAILABLE,
      Error::SurrealDBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::ValidationErrors(_) => StatusCode::BAD_REQUEST,
      Error::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
      Error::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
      Error::ConfigError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      Error::TokenGenerationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
      _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }
}

