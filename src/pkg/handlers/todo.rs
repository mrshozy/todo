use crate::pkg::{
    handlers::error::ApiResult,
    models::todo::{
        ApiCreateTodo,
        ApiUpdateTodo,
    },
    repository::models::todo::TodoBmc,
    state::TodoState,
};
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{
        delete,
        get,
        post,
        put,
    },
    Extension,
    Json,
    Router,
};
use serde_json::json;

pub fn todo_router() -> Router {
    Router::new()
        .route("/", post(create_todo))
        .route("/:id", get(get_todo))
        .route("/:id", put(update_todo))
        .route("/:id", delete(delete_todo))
}

async fn create_todo(
    Extension(state): Extension<TodoState>,
    Json(payload): Json<ApiCreateTodo>,
) -> ApiResult {
    let todo = TodoBmc::create(state.get_manager(), payload.into()).await?;
    Ok(Json(todo).into_response())
}

async fn update_todo(
    Extension(state): Extension<TodoState>,
    Path(id): Path<String>,
    Json(payload): Json<ApiUpdateTodo>,
) -> ApiResult {
    TodoBmc::update(state.get_manager(), id, payload.into()).await?;
    Ok(Json(json!({"status": "updated"})).into_response())
}

async fn get_todo(Extension(state): Extension<TodoState>, Path(id): Path<String>) -> ApiResult {
    let todo = TodoBmc::get(state.get_manager(), id).await?;
    Ok(Json(todo).into_response())
}

async fn delete_todo(Extension(state): Extension<TodoState>, Path(id): Path<String>) -> ApiResult {
    TodoBmc::delete(state.get_manager(), id).await?;
    Ok(Json(json!({"status": "deleted"})).into_response())
}
