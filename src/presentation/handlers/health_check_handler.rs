use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/")]
pub async fn health_check_handler() -> impl Responder {
  HttpResponse::Ok().json(json!({
    "status": "I'm alive!"
  }))
}
