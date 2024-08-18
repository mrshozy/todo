use crate::pkg::repository::models::todo::{CreateTodo, UpdateTodo};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiCreateTodo {
    title: String,
    description: Option<String>,
    due_date: Option<NaiveDateTime>,
    priority: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiUpdateTodo {
    title: String,
    description: Option<String>,
    due_date: Option<NaiveDateTime>,
    priority: i32,
    completed: bool,
}

impl From<ApiCreateTodo> for CreateTodo {
    fn from(value: ApiCreateTodo) -> Self {
        CreateTodo::new(
            value.title,
            value.description,
            value.due_date,
            value.priority,
        )
    }
}

impl From<ApiUpdateTodo> for UpdateTodo {
    fn from(value: ApiUpdateTodo) -> Self {
        UpdateTodo::new(
            value.title,
            value.description,
            value.due_date,
            value.priority,
            value.completed,
        )
    }
}
