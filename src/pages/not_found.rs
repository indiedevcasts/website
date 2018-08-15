use actix_web::{HttpRequest, HttpResponse, Result};
use actix_web::http::{StatusCode};

/// 404 handler
pub fn init(_req: &HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::NOT_FOUND)
      .content_type("text/html; charset=utf-8")
      .body(include_str!("../../static/landing-page/404.html"))
    )
}
