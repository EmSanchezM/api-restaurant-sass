use actix_web::web;
use crate::presentation::handlers::profile_handler::{
  create_profile_handler, get_profile_handler, get_profile_by_id_handler, update_profile_handler, delete_profile_handler
};

//TODO: Agregar middleware para validar token estas rutas seran disponibles solo para usuario autenticado
pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api/v1/profile")
      .service(create_profile_handler)
      .service(get_profile_handler)
      .service(get_profile_by_id_handler)
      .service(update_profile_handler)
      .service(delete_profile_handler)
  );
}