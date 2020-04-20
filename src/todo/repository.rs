use crate::todo::TodoItem;
use uuid::Uuid;

use sqlx::pool::Pool;
use sqlx::postgres::{PgConnection, PgPool, PgQueryAs};
use std::env;

pub struct TodoRepositoryImpl {
    pool: Pool<PgConnection>,
}

impl TodoRepositoryImpl {
    pub async fn new() -> Self {
        let pool: Pool<PgConnection> = PgPool::builder()
            .max_size(5) // maximum number of connections in the pool
            // postgresql://<user>[:<password>]@<host>[:<port>]/<database>[?sslmode=<ssl-mode>[&sslcrootcert=<path>
            .build(env::var("DATABASE_URL").as_ref().unwrap())
            .await
            .unwrap();
        Self { pool }
    }
}

impl TodoRepositoryImpl {
    pub async fn create(&self, item: TodoItem) -> Result<Uuid, sqlx::Error> {
        // sqlx::query_as(
        //     r#"
        //     INSERT INTO todo_item ( id, name, content, created_time )
        //     VALUES ( $1, $2, $3, $4 )
        //     RETURNING id"#,
        // )
        // .bind(item.id)
        // .bind(item.name)
        // .bind(item.content)
        // .bind(item.created_time)
        // .fetch_one(&self.pool)
        // .await
        // .map(|r: (Uuid,)| r.0)
        Ok(Uuid::new_v4())
    }

    pub async fn get_all(&self) -> Result<Vec<TodoItem>, sqlx::Error> {
        // sqlx::query_as("SELECT * FROM todo_item")
        //     // sqlx::query_as!(TodoItem, "SELECT * FROM todo_item")
        //     .fetch_all(&self.pool)
        //     .await
        Ok(vec![])
    }

    pub async fn get_one(&self, id: Uuid) -> Result<TodoItem, sqlx::Error> {
        // sqlx::query_as(
        //     r#"
        //     SELECT * 
        //     FROM todo_item 
        //     WHERE id = $1
        //     "#,
        // )
        // .bind(id)
        // .fetch_one(&self.pool)
        // .await
        Ok(TodoItem::new("fake".to_string(), "I have no content".to_string()))
    }
}
