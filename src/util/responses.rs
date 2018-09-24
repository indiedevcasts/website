use askama::Template;
use actix_web::HttpResponse;

#[inline(always)]
pub fn render(template: &Template) -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(template.render().unwrap())
}