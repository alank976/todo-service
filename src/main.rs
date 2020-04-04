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
            .data(injection::init())
            .configure(todo::api::config)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

mod injection {
    use super::todo::domain::{TodoService, TodoServiceImpl};

    pub fn init() -> Container {
        Container::init(Box::new(TodoServiceImpl::new()))
    }
    pub struct Container {
        pub todo_service: Box<dyn TodoService>,
    }

    impl Container {
        pub fn init(todo_service: Box<dyn TodoService>) -> Self {
            Self { todo_service }
        }
    }
}

#[cfg(test)]
mod test_integration {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test() {
        let mut app = test::init_service(
            App::new()
                .data(injection::init())
                .configure(todo::api::config),
        )
        .await;
        let req = test::TestRequest::get().uri("/todos").to_request();
        let todo_items: Vec<todo::api::ItemDto> = test::read_response_json(&mut app, req).await;
        assert_eq!(1, todo_items.len());

        let request_body = todo::api::ItemDto {
            id: None,
            name: "something".to_string(),
            description: "nothing".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&request_body)
            .to_request();
        let resp: todo::api::ItemDto = test::read_response_json(&mut app, req).await;
        assert!(resp.id.is_some());
    }
}
