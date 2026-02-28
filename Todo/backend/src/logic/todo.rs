use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePool, FromRow, Row};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct UserRecord {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Project {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Task {
    pub id: i64,
    pub project_id: i64,
    pub parent_id: Option<i64>, 
    pub title: String,
    pub description: Option<String>,
    pub priority: i32,
    pub completed: bool,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, FromRow)]
pub struct Comment {
    pub id: i64,
    pub task_id: i64,
    pub user_id: i64,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

pub async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query("PRAGMA foreign_keys = ON;").execute(pool).await?;

    // 1. Users Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        )"
    ).execute(pool).await?;

    // 2. Projects Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS projects (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            color TEXT,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;

    // 3. Tasks Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_id INTEGER NOT NULL,
            parent_id INTEGER,
            title TEXT NOT NULL,
            description TEXT,
            priority INTEGER DEFAULT 4,
            completed BOOLEAN NOT NULL DEFAULT 0,
            due_date DATETIME,
            FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
            FOREIGN KEY (parent_id) REFERENCES tasks (id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;

    // 4. Comments Table
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS comments (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            user_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users (id) ON DELETE CASCADE
        )"
    ).execute(pool).await?;

    Ok(())
}

// --- Users ---

pub async fn create_user(pool: &SqlitePool, username: &str, password_hash: &str) -> Result<i64, sqlx::Error> {
    let id = sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
        .bind(username)
        .bind(password_hash)
        .execute(pool)
        .await?
        .last_insert_rowid();
    Ok(id)
}

pub async fn find_user_by_username(pool: &SqlitePool, username: &str) -> Result<Option<UserRecord>, sqlx::Error> {
    sqlx::query_as::<_, UserRecord>("SELECT * FROM users WHERE username = ?")
        .bind(username)
        .fetch_optional(pool)
        .await
}

// --- Projects ---

pub async fn create_project(pool: &SqlitePool, user_id: i64, name: &str, color: Option<&str>) -> Result<Project, sqlx::Error> {
    let color_to_save = color.unwrap_or("Blue");
    
    let id = sqlx::query("INSERT INTO projects (user_id, name, color) VALUES (?, ?, ?)")
        .bind(user_id)
        .bind(name)
        .bind(color_to_save)
        .execute(pool)
        .await?
        .last_insert_rowid();

    Ok(Project { 
        id, 
        user_id, 
        name: name.to_string(), 
        color: Some(color_to_save.to_string()) 
    })
}

pub async fn list_projects(pool: &SqlitePool, user_id: i64) -> Result<Vec<Project>, sqlx::Error> {
    sqlx::query_as::<_, Project>("SELECT * FROM projects WHERE user_id = ?")
        .bind(user_id)
        .fetch_all(pool)
        .await
}

pub async fn project_exists(pool: &SqlitePool, project_id: i64, user_id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("SELECT 1 FROM projects WHERE id = ? AND user_id = ?")
        .bind(project_id)
        .bind(user_id)
        .fetch_optional(pool)
        .await?;
    
    Ok(result.is_some())
}

// --- Tasks ---

pub async fn create_task(
    pool: &SqlitePool, 
    project_id: i64, 
    title: &str, 
    priority: i32,
    due_date: Option<DateTime<Utc>>
) -> Result<Task, sqlx::Error> {
    let id = sqlx::query(
        "INSERT INTO tasks (project_id, title, priority, due_date) VALUES (?, ?, ?, ?)"
    )
    .bind(project_id)
    .bind(title)
    .bind(priority)
    .bind(due_date)
    .execute(pool)
    .await?
    .last_insert_rowid();

    Ok(Task {
        id,
        project_id,
        parent_id: None,
        title: title.to_string(),
        description: None,
        priority,
        completed: false,
        due_date,
    })
}

pub async fn list_tasks(pool: &SqlitePool, project_id: i64) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as::<_, Task>("SELECT * FROM tasks WHERE project_id = ?")
        .bind(project_id)
        .fetch_all(pool)
        .await
}

pub async fn complete_task(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("UPDATE tasks SET completed = 1 WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected() > 0)
}

// --- Comments ---

pub async fn add_comment(pool: &SqlitePool, task_id: i64, user_id: i64, content: &str) -> Result<Comment, sqlx::Error> {
    let res = sqlx::query(
        "INSERT INTO comments (task_id, user_id, content) VALUES (?, ?, ?) RETURNING id, created_at"
    )
    .bind(task_id)
    .bind(user_id)
    .bind(content)
    .fetch_one(pool)
    .await?;

    Ok(Comment {
        id: res.get(0),
        task_id,
        user_id,
        content: content.to_string(),
        created_at: res.get(1),
    })
}
