use actix_web::{error, web, HttpResponse, Responder};

use serde::{Deserialize, Serialize};
use validator::Validate;

async fn get_all() -> impl Responder {
    HttpResponse::Ok().json(vec![Item {
        id: "111".to_string(),
        name: "hello".to_string(),
        description: "description=n/a".to_string(),
    }])
}

async fn create(item: web::Json<Item>) -> Result<HttpResponse, error::Error> {
    let item = item.into_inner();
    item.validate().map_err(error::ErrorBadRequest)?;

    Ok(HttpResponse::Ok().json(item))
}

async fn get_one(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(Item {
        id: id.into_inner(),
        name: "hello".to_string(),
        description: "description=n/a".to_string(),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("", web::get().to(get_all))
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_one))
                    .route(web::post().to(create)),
            ),
    );
}

#[derive(Debug, Validate, Deserialize, Serialize)]
struct Item {
    id: String,
    #[validate(length(min = 5))]
    name: String,
    description: String,
}
