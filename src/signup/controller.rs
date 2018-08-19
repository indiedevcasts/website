use actix_web::{http, HttpRequest, HttpResponse, Form, Result};
use signup::model::SignUp;

pub fn signup(params: Form<SignUp>) -> Result<HttpResponse> {
  println!("Params : {:?}", params);
  Ok(HttpResponse::build(http::StatusCode::OK)
    .content_type("text/plain")
    .body(format!("{:?}", params))
  )
}
