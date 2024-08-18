use crate::pkg::repository::{crud, crud::DbBmc, error::DatabaseResult, manager::ModelManager};
use chrono::NaiveDateTime;
use modql::field::Fields;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow, Fields)]
pub struct Todo {
    id: String,
    title: String,
    description: Option<String>,
    due_date: Option<NaiveDateTime>,
    completed: bool,
    priority: i32,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow, Fields)]
pub struct CreateTodo {
    id: String,
    title: String,
    description: Option<String>,
    due_date: Option<NaiveDateTime>,
    completed: bool,
    priority: i32,
}

impl CreateTodo {
    pub fn new(
        title: String,
        description: Option<String>,
        due_date: Option<NaiveDateTime>,
        priority: i32,
    ) -> Self {
        Self {
            completed: false,
            id: Uuid::now_v7().to_string(),
            title,
            priority,
            due_date,
            description,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow, Fields)]
pub struct UpdateTodo {
    title: String,
    description: Option<String>,
    due_date: Option<NaiveDateTime>,
    completed: bool,
    priority: i32,
}

impl UpdateTodo {
    pub fn new(
        title: String,
        description: Option<String>,
        due_date: Option<NaiveDateTime>,
        priority: i32,
        completed: bool,
    ) -> Self {
        Self {
            completed,
            title,
            priority,
            due_date,
            description,
        }
    }
}

pub struct TodoBmc;

impl DbBmc for TodoBmc {
    const TABLE: &'static str = "todos";
}

impl TodoBmc {
    pub async fn create(mm: &ModelManager, todo: CreateTodo) -> DatabaseResult<Todo> {
        crud::create::create::<Self, _, Todo>(mm, todo).await
    }

    pub async fn get(mm: &ModelManager, id: String) -> DatabaseResult<Todo> {
        crud::get::get::<Self, _>(mm, id).await
    }

    pub async fn update(mm: &ModelManager, id: String, project: UpdateTodo) -> DatabaseResult<()> {
        crud::update::update::<Self, _>(mm, id, project).await
    }

    pub async fn delete(mm: &ModelManager, id: String) -> DatabaseResult<()> {
        crud::delete::delete::<Self>(mm, id).await
    }
}
