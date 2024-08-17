use crate::pkg::handlers::{
    ping::ping_router,
    todo::todo_router,
};
use axum::Router;

mod error;
mod ping;
mod todo;

pub fn handlers_router() -> Router {
    Router::new().nest("/todo", todo_router()).nest("/ping", ping_router())
}
