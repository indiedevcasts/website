#![allow(unused_variables)]

extern crate actix;
extern crate actix_web;
extern crate env_logger;

use actix_web::http::{header, Method, StatusCode};
use actix_web::{
    fs, middleware, pred,
    server, App, HttpRequest,
    HttpResponse, Result
};

use std::{env};

fn not_found(req: &HttpRequest) -> Result<HttpResponse> {
	Ok(HttpResponse::build(StatusCode::NOT_FOUND)
        .content_type("text/html; charset=utf-8")
		.body(include_str!("../static/landing-page/404.html")))
}

fn landing_page(req: &HttpRequest) -> Result<HttpResponse> {
	Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
		.body(include_str!("../static/landing-page/index.html")))
}

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    // env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
	
	let sys = actix::System::new("indiedevcasts-landing-page");

 	let addr = server::new(
    	|| App::new()
        	// enable logger
            .middleware(middleware::Logger::default())
            // static files
			.handler("/static", fs::StaticFiles::new("static").unwrap())
            .resource("/index.html", |r| r.f(landing_page))
            // redirect to index.html
            .resource("/", |r| r.method(Method::GET).f(|req| {
                HttpResponse::Found()
                    .header(header::LOCATION, "/index.html")
                    .finish()
			}))
			.default_resource(|r| {
                // 404 for GET request
                r.method(Method::GET).f(not_found);

                // all requests that are not `GET`
                r.route().filter(pred::Not(pred::Get())).f(
                    |req| HttpResponse::MethodNotAllowed());
            }))
        	.bind("127.0.0.1:8080").expect("Can not bind to 127.0.0.1:8080")
        	.shutdown_timeout(0)
			.start();
	
	println!("Starting http server: 127.0.0.1:8080");
	let _ = sys.run();
}
