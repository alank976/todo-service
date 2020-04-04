#[macro_use]
extern crate validator_derive;
extern crate validator;

use std::io;

mod todo;
use actix_web::{middleware::Logger, App, HttpServer};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "debug,my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .configure(todo::api::config)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
