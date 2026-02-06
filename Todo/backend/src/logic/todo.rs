use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow};

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Todo {
    pub id: u32,
    pub task: String,
    pub completed: bool,
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT 0
        )"
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn load_all(pool: &SqlitePool) -> Result<Vec<Todo>, sqlx::Error> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, task, completed FROM todos")
        .fetch_all(pool)
        .await?;
    Ok(todos)
}

pub async fn find_by_id(pool: &SqlitePool, id: u32) -> Result<Option<Todo>, sqlx::Error> {
    let todo = sqlx::query_as::<_, Todo>("SELECT id, task, completed FROM todos WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    Ok(todo)
}

pub async fn create(pool: &SqlitePool, task: &str) -> Result<Todo, sqlx::Error> {
    let id = sqlx::query("INSERT INTO todos (task, completed) VALUES (?, 0)")
        .bind(task)
        .execute(pool)
        .await?
        .last_insert_rowid();

    Ok(Todo {
        id: id as u32,
        task: task.to_string(),
        completed: false,
    })
}

pub async fn complete(pool: &SqlitePool, id: u32) -> Result<Option<Todo>, sqlx::Error> {
    sqlx::query("UPDATE todos SET completed = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    find_by_id(pool, id).await
}
