use crate::pkg::{
    handlers::handlers_router,
    router::{
        cors::cors_layer,
        tracing::tracing,
    },
    state::TodoState,
};
use axum::{
    Extension,
    Router,
};

mod cors;
mod tracing;

pub fn create_router(state: TodoState) -> Router {
    Router::new()
        .merge(handlers_router())
        .layer(cors_layer())
        .layer(tracing())
        .layer(Extension(state))
}
