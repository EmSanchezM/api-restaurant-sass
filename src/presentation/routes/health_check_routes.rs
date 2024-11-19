use actix_web::web;
use crate::presentation::handlers::health_check_handler::health_check_handler;

pub fn routes(config: &mut web::ServiceConfig) {
  config.service(
    web::scope("/api/v1/health-check")
      .service(health_check_handler)
  );
}