use crate::injection::Container;
use actix_web::{error, web, web::Data, HttpResponse, Responder};
use uuid::Uuid;

use super::domain::TodoItem;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[cfg(test)]
use mockiato::mockable;

#[cfg_attr(test, mockable)]
trait Greeter {
    fn greet(&self, name: &str) -> String;
}

async fn get_all(app_container: Data<Container>) -> impl Responder {
    HttpResponse::Ok().json(
        (&app_container.todo_service)
            .get_all()
            .into_iter()
            .map(|i| ItemDto::from_domain(i))
            .collect::<Vec<ItemDto>>(),
    )
}

async fn create(
    item: web::Json<ItemDto>,
    app_container: Data<Container>,
) -> Result<HttpResponse, error::Error> {
    let dto = item.into_inner();
    dto.validate().map_err(error::ErrorBadRequest)?;

    (&app_container.todo_service)
        .create(dto.to_domain())
        .ok_or(error::ErrorInternalServerError("failed to create"))
        .map(ItemDto::from_domain)
        .map(|dto| HttpResponse::Ok().json(dto))
}

async fn get_one(
    id: web::Path<Uuid>,
    app_container: Data<Container>,
) -> Result<HttpResponse, error::Error> {
    (&app_container.todo_service)
        .get_one(id.into_inner())
        .ok_or(error::ErrorNotFound("item not found"))
        .map(ItemDto::from_domain)
        .map(|dto| HttpResponse::Ok().json(dto))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .route("", web::get().to(get_all))
            .route("", web::post().to(create))
            .service(web::resource("/{id}").route(web::get().to(get_one))),
    );
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct ItemDto {
    pub id: Option<Uuid>,
    #[validate(length(min = 5))]
    pub name: String,
    pub description: String,
}

impl ItemDto {
    fn from_domain(domain: TodoItem) -> Self {
        Self {
            id: domain.id,
            name: domain.name,
            description: domain.content,
        }
    }

    fn to_domain(self) -> TodoItem {
        TodoItem::new(self.name, self.description)
    }
}
