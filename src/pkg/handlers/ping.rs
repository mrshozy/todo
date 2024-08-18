use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};

pub fn ping_router() -> Router {
    Router::new().route("/", get(ping))
}

async fn ping() -> impl IntoResponse {
    (StatusCode::OK, "pong").into_response()
}
