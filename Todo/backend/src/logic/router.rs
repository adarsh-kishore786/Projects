use axum::{
    Router, extract::State, routing::{get, post, delete, patch}
};
use sqlx::SqlitePool;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::logic::auth::{self, Claims};
use crate::logic::todo;
use crate::logic::error::{AppError, ServerError, Path, Json};

pub type SharedState = SqlitePool;

pub fn get_router(state: SharedState) -> Router {
    Router::new()
        .route("/signup", post(auth::signup))
        .route("/login", post(auth::login))
        // Projects
        .route("/projects", get(get_projects))
        .route("/projects", post(create_project))
        .route("/projects/:project_id", get(get_project))
        .route("/projects/:project_id", patch(edit_project))
        .route("/projects/:project_id", delete(delete_project))
        // Tasks
        .route("/projects/:project_id/tasks", get(get_tasks))
        .route("/projects/:project_id/tasks", post(create_task))
        .route("/tasks/:id", get(get_task))
        .route("/tasks/:id", patch(edit_task))
        .route("/tasks/:id", delete(delete_task))
        .route("/tasks/:id/complete", post(complete_task))
        // Comments
        .route("/tasks/:id/comments", get(get_comments))
        .route("/tasks/:id/comments", post(add_comment))
        .route("/comments/:id", patch(edit_comment))
        .route("/comments/:id", delete(delete_comment))
        .with_state(state)
}

// --- Projects ---

#[derive(Deserialize)]
struct CreateProject {
    name: String,
    color: Option<String>
}

#[derive(Deserialize)]
struct EditProject {
    name: Option<String>,
    color: Option<String>
}

async fn create_project(
    claims: Claims,
    State(pool): State<SharedState>,
    Json(input): Json<CreateProject>,
) -> Result<Json<todo::Project>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(db_err)?;
    let project = todo::create_project(
        &pool, 
        user_id, 
        &input.name, 
        input.color.as_deref()
    ).await.map_err(db_err)?;
    Ok(Json(project))
}

async fn get_projects(
    claims: Claims,
    State(pool): State<SharedState>,
) -> Result<Json<Vec<todo::Project>>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(db_err)?;
    let projects = todo::list_projects(&pool, user_id).await.map_err(db_err)?;
    Ok(Json(projects))
}

async fn get_project(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(project_id): Path<i64>,
) -> Result<Json<todo::Project>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(db_err)?;
    let project = todo::get_project(&pool, project_id, user_id).await;

    if let Ok(p) = project {
        return Ok(Json(p));
    }

    return Err(ServerError::NotFound("Project").into());
}

async fn edit_project(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(project_id): Path<i64>,
    Json(input): Json<EditProject>
) -> Result<Json<todo::Project>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(db_err)?;
    
    let result = todo::edit_project(
        &pool, 
        project_id, 
        user_id, 
        input.name.as_deref(), 
        input.color.as_deref()
    ).await.map_err(|e| {
        if let sqlx::Error::RowNotFound = e {
            return ServerError::NotFound("Project");
        }
        db_err(e)
    })?;

    Ok(Json(result))
}

async fn delete_project(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(project_id): Path<i64>,
) -> Result<(), AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(db_err)?;
    let success = todo::delete_project(&pool, project_id, user_id).await.map_err(db_err)?;
    
    if !success {
        return Err(ServerError::NotFound("Project").into());
    }
    
    Ok(())
}

// --- Tasks ---

#[derive(Deserialize)]
struct CreateTask {
    title: String,
    priority: Option<i32>,
    due_date: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
struct EditTask {
    title: Option<String>,
    priority: Option<i32>,
    completed: Option<bool>,
    due_date: Option<DateTime<Utc>>,
}

async fn create_task(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(project_id): Path<i64>,
    Json(input): Json<CreateTask>,
) -> Result<Json<todo::Task>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(|_| ServerError::Internal)?;
    
    if !todo::project_exists(&pool, project_id, user_id).await.map_err(db_err)? {
        return Err(ServerError::NotFound("Project").into());
    }

    let task = todo::create_task(
        &pool, 
        project_id, 
        &input.title, 
        input.priority.unwrap_or(4), 
        input.due_date
    ).await.map_err(db_err)?;
    
    Ok(Json(task))
}

async fn get_task(
    _claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<todo::Task>, AppError> {
    let task = todo::get_task(&pool, id).await.map_err(|e| {
        if let sqlx::Error::RowNotFound = e {
            return ServerError::NotFound("Task");
        }
        db_err(e)
    })?;
    
    Ok(Json(task))
}

async fn edit_task(
    _claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
    Json(input): Json<EditTask>,
) -> Result<Json<todo::Task>, AppError> {
    let task = todo::edit_task(
        &pool, 
        id, 
        input.title.as_deref(), 
        input.priority, 
        input.completed, 
        input.due_date
    ).await.map_err(|e| {
        if let sqlx::Error::RowNotFound = e {
            return ServerError::NotFound("Task");
        }
        db_err(e)
    })?;
    
    Ok(Json(task))
}

async fn delete_task(
    _claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let success = todo::delete_task(&pool, id).await.map_err(db_err)?;
    
    if !success {
        return Err(ServerError::NotFound("Task").into());
    }
    
    Ok(())
}

async fn get_tasks(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(project_id): Path<i64>,
) -> Result<Json<Vec<todo::Task>>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(|_| ServerError::Internal)?;
    
    if !todo::project_exists(&pool, project_id, user_id).await.map_err(db_err)? {
        return Err(ServerError::NotFound("Project").into());
    }

    let tasks = todo::list_tasks(&pool, project_id).await.map_err(db_err)?;
    Ok(Json(tasks))
}

async fn complete_task(
    _claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let success = todo::complete_task(&pool, id).await.map_err(db_err)?;
    
    if !success {
        return Err(ServerError::NotFound("Task").into());
    }
    
    Ok(())
}

// --- Comments ---

#[derive(Deserialize)]
struct CreateComment {
    content: String,
}

async fn get_comments(
    _claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<Json<Vec<todo::Comment>>, AppError> {
    let comments = todo::list_comments(&pool, id).await.map_err(db_err)?;
    Ok(Json(comments))
}

async fn add_comment(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
    Json(input): Json<CreateComment>,
) -> Result<Json<todo::Comment>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(|_| ServerError::Internal)?;
    
    // add_comment will fail with DB error if task id is invalid due to FK
    let comment = todo::add_comment(&pool, id, user_id, &input.content).await.map_err(|e| {
        // If it's a foreign key error, return 404
        if let Some(db_err) = e.as_database_error() {
            if db_err.is_foreign_key_violation() {
                return ServerError::NotFound("Task");
            }
        }
        db_err(e)
    })?;
    
    Ok(Json(comment))
}

async fn edit_comment(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
    Json(input): Json<CreateComment>,
) -> Result<Json<todo::Comment>, AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(db_err)?;
    let comment = todo::edit_comment(&pool, id, user_id, &input.content).await.map_err(|e| {
        if let sqlx::Error::RowNotFound = e {
            return ServerError::NotFound("Comment");
        }
        db_err(e)
    })?;
    
    Ok(Json(comment))
}

async fn delete_comment(
    claims: Claims,
    State(pool): State<SharedState>,
    Path(id): Path<i64>,
) -> Result<(), AppError> {
    let user_id = claims.sub.parse::<i64>().map_err(db_err)?;
    let success = todo::delete_comment(&pool, id, user_id).await.map_err(db_err)?;
    
    if !success {
        return Err(ServerError::NotFound("Comment").into());
    }
    
    Ok(())
}

// --- Helpers ---

fn db_err<E: std::fmt::Display>(err: E) -> ServerError {
    eprintln!("Database Error: {}", err);
    ServerError::Internal
}
