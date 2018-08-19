mod controller;
mod model;

use actix_web::App;
use actix_web::http::Method;
use self::controller as ctrl;

pub fn init(app: App) -> App {
  app.resource("/signup", |r| {
    r.method(Method::POST).with(ctrl::signup)
  })
}
