use actix_web::web;

use crate::presentation::handlers::users_handler::{
  get_all_users_handler,
  verification_user_handler,
  change_password_handler,
  disable_user_handler
};
//TODO: Separar rutas get all y disable user for super admin only y verification user and change password for usuarios autenticados
pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api/v1/users")
      .service(get_all_users_handler)
      .service(verification_user_handler)
      .service(change_password_handler)
      .service(disable_user_handler)
  );
}