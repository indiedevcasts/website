mod controller;
mod model;

use actix_web::App;
use self::controller as ctrl;

pub fn init(app: App) -> App {
  app.resource("/signup", |r| r.f(ctrl::signup))
}
