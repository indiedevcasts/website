mod not_found;
mod landing_page;

use actix_web::{App, fs, middleware, HttpResponse};
use actix_web::http::{header, Method};

pub fn init(app: App) -> App {
  app
  // logger
  .middleware(middleware::Logger::default())
  // static files
  .handler("/static", fs::StaticFiles::new("static").unwrap())
  // index page
  .resource("/index.html", |r| r.f(landing_page::init))
  .resource("/", |r| r.method(Method::GET).f(|_req| {
    HttpResponse::Found()
      .header(header::LOCATION, "/index.html")
      .finish()
  }))
  .default_resource(|r| {
    // 404 on all methods
    r.f(not_found::init);
  })
}
