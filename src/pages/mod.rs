mod not_found;
mod landing_page;

use actix_web::{App, fs, middleware, HttpRequest, HttpResponse};
use actix_web::http::{header, Method};
use askama::Template;
use util::responses::render;

#[derive(Template)]
#[template(path = "index.html")]
pub struct LandingPage {}

pub fn init(app: App) -> App {
  app
  .middleware(middleware::Logger::default())
  .resource("/", |r| r.method(Method::GET).f(|_req| render(&LandingPage {})))
  .default_resource(|r| {
    // 404 on all methods
    r.f(not_found::init);
  })
}
