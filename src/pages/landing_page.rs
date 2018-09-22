use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::http::{StatusCode};

/// landing page handler
pub fn init(_req: &HttpRequest) -> Result<HttpResponse> {
  Ok(HttpResponse::build(StatusCode::OK)
    .content_type("text/html; charset=utf-8")
    .body(include_str!("../../static/landing-page/index.html"))
  )
}
