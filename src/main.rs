#[macro_use]
extern crate validator_derive;
extern crate validator;

use std::io;
use std::sync::Arc;

mod todo;

use actix_web::{middleware::Logger, App, HttpServer};
use todo::{TodoRepositoryImpl, TodoServiceImpl};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("DATABASE_URL", "postgresql://todo:todo@localhost:5432/todo");
    std::env::set_var("RUST_LOG", "debug,my_errors=debug,actix_web=info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let app_data = Arc::new(Components::new().await);
    
    HttpServer::new(move || {
        App::new()
            .data(Arc::clone(&app_data))
            .configure(todo::handler::config)
            .wrap(Logger::default())
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}

pub struct Components {
    todo_service: TodoServiceImpl,
}

impl Components {
    async fn new() -> Self {
        let todo_repo = TodoRepositoryImpl::new().await;
        let todo_service = TodoServiceImpl::new(todo_repo);
        Self { todo_service }
    }
}

#[cfg(test)]
mod test_integration {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test() {
        let app_data = Arc::new(Components::new().await);
        let mut app = test::init_service(
            App::new()
                .data(app_data.clone())
                .configure(todo::handler::config),
        )
        .await;
        let req = test::TestRequest::get().uri("/todos").to_request();
        let todo_items: Vec<todo::handler::ItemDto> = test::read_response_json(&mut app, req).await;
        assert_eq!(1, todo_items.len());

        let request_body = todo::handler::ItemDto {
            id: None,
            name: "something".to_string(),
            description: "nothing".to_string(),
        };
        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&request_body)
            .to_request();
        let resp: todo::handler::ItemDto = test::read_response_json(&mut app, req).await;
        assert!(resp.id.is_some());
    }
}
