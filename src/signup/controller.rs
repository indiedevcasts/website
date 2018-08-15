use actix_web::{HttpRequest, HttpResponse};

pub fn signup(_req: HttpRequest) -> HttpResponse {
  println!("Received a signup request");
  HttpResponse::Ok()
    .content_type("application/json")
    .body("Received a signup request")
}
