#[macro_use]
extern crate validator_derive;
extern crate validator;

use std::io;

mod todo;
use actix_web::{App, HttpServer};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| App::new().configure(todo::api::config))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}
