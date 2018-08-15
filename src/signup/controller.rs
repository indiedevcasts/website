use actix_web::{HttpRequest, HttpResponse};

pub fn signup(_req: HttpRequest) -> HttpResponse {
  HttpResponse::Ok()
    .content_type("application/json")
    .body("Signup request")
}
