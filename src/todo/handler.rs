use actix_web::{error, web, web::Data, HttpResponse};
use uuid::Uuid;

use crate::todo::TodoItem;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::Components;
use std::sync::Arc;

async fn get_all(components: Data<Arc<Components>>) -> Result<HttpResponse, error::Error> {
    components
        .todo_service
        .get_all()
        .await
        .map_err(error::ErrorInternalServerError)
        .map(|items| {
            items
                .into_iter()
                .map(|i| ItemDto::from_domain(i))
                .collect::<Vec<ItemDto>>()
        })
        .map(|dtos| HttpResponse::Ok().json(dtos))
}

async fn create(
    item: web::Json<ItemDto>,
    components: Data<Arc<Components>>,
) -> Result<HttpResponse, error::Error> {
    let dto = item.into_inner();
    dto.validate().map_err(error::ErrorBadRequest)?;
    components
        .todo_service
        .create(dto.to_domain())
        .await
        .map_err(error::ErrorInternalServerError)
        .map(|created_id| HttpResponse::Ok().json(created_id))
}

async fn get_one(
    id: web::Path<Uuid>,
    components: Data<Arc<Components>>,
) -> Result<HttpResponse, error::Error> {
    components
        .todo_service
        .get_one(id.into_inner())
        .await
        .map_err(|e: sqlx::Error| match e {
            sqlx::Error::RowNotFound => error::ErrorNotFound("not found"),
            e @ _ => error::ErrorInternalServerError(e),
        })
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
