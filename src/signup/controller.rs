use actix_web::{AsyncResponder, HttpMessage, HttpRequest, HttpResponse, FutureResponse};
use futures::future::Future;
use qs;
use signup::model::PreSignUp;
use percent_encoding::percent_decode;

pub fn signup(req: &HttpRequest) -> FutureResponse<HttpResponse> {
  req.body()
    .from_err()
    .and_then(|body| {
      let body_str = percent_decode(&body).decode_utf8().unwrap();
      let signup: PreSignUp = qs::from_str(&body_str).unwrap();
      Ok(HttpResponse::Ok().body("".to_string()).into())
    })
    .responder()
}
