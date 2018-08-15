mod controller;

use actix_web::App;
use actix_web::http::Method;
use self::controller as ctrl;

pub fn init(app: App) -> App {
  app.route("/signup", Method::POST, ctrl::signup)
}
