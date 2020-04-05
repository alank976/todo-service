use crate::todo::TodoRepositoryImpl;
use uuid::Uuid;

use log::debug;

pub struct TodoServiceImpl {
    repo: TodoRepositoryImpl,
}

impl TodoServiceImpl {
    pub fn new(repo: TodoRepositoryImpl) -> Self {
        Self { repo }
    }
}

impl TodoServiceImpl {
    pub async fn get_all(&self) -> Result<Vec<TodoItem>, sqlx::Error> {
        debug!("Getting all items");
        self.repo.get_all().await
    }

    pub async fn get_one(&self, id: Uuid) -> Result<TodoItem, sqlx::Error> {
        debug!("get just one item id={:?}", id);
        self.repo.get_one(id).await
    }

    pub async fn create(&self, item: TodoItem) -> Result<Uuid, sqlx::Error> {
        debug!("Creating item named {}", item.name);
        self.repo.create(item).await
    }
}

#[derive(Clone, Debug, sqlx::FromRow)]
pub struct TodoItem {
    pub id: Option<Uuid>,
    pub name: String,
    pub content: String,
    pub created_time: time::PrimitiveDateTime,
}

impl TodoItem {
    pub fn new(name: String, content: String) -> Self {
        Self {
            id: Some(Uuid::new_v4()),
            name,
            content,
            created_time: time::PrimitiveDateTime::now(),
        }
    }
}
