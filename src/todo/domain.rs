use std::time;
use uuid::Uuid;

use log::debug;

#[cfg(test)]
use mockiato::mockable;

#[cfg_attr(test, mockable)]
pub trait TodoService {
    fn get_all(&self) -> Vec<TodoItem>;
    fn get_one(&self, id: Uuid) -> Option<TodoItem>;
    fn create(&self, item: TodoItem) -> Option<TodoItem>;
}

pub struct TodoServiceImpl {}

impl TodoServiceImpl {
    pub fn new() -> Self {
        Self {}
    }
}
impl TodoService for TodoServiceImpl {
    fn get_all(&self) -> Vec<TodoItem> {
        debug!("getting all items");
        vec![TodoItem::new("random".to_string(), "nth".to_string())]
    }

    fn get_one(&self, id: Uuid) -> Option<TodoItem> {
        debug!("get just one item id={:?}", id);
        None
    }

    fn create(&self, item: TodoItem) -> Option<TodoItem> {
        debug!("Creating item named {}", item.name);
        Some(TodoItem::new_from(item))
    }
}

#[derive(Clone)]
pub struct TodoItem {
    pub id: Option<Uuid>,
    pub name: String,
    pub content: String,
    pub created_time: time::Instant,
}

impl TodoItem {
    pub fn new(name: String, content: String) -> Self {
        Self {
            id: Some(Uuid::new_v4()),
            name,
            content,
            created_time: time::Instant::now(),
        }
    }

    fn new_from(other: Self) -> Self {
        Self::new(other.name, other.content)
    }
}
