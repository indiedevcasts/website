extern crate actix;
extern crate actix_web;
extern crate env_logger;

#[macro_use]
extern crate serde_derive;

// TODO : wait for email validator debug
// #[macro_use]
// extern crate validator_derive;
// extern crate validator;

mod pages;
mod signup;

use actix_web::{server, App};
use std::{env};

fn main() {
    env::set_var("RUST_LOG", "actix_web=debug");
    // env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let sys = actix::System::new("indiedevcasts-landing-page");

    let _addr = server::new(|| {
        let mut app = App::new();

        app = pages::init(app);
        app = signup::init(app);

        app
    })
    .bind("127.0.0.1:8080")
    .expect("Can not bind to 127.0.0.1:8080")
    .shutdown_timeout(0)
    .start();

    println!("Starting the server on 127.0.0.1:8080");
    let _ = sys.run();
}
