use axum::{routing::get, Router};
use tower_http::trace::TraceLayer;

async fn handler() -> &'static str {
    "Hello, World!"
}

pub fn build() -> Router {
    Router::new()
        .route("/", get(handler))
        .layer(TraceLayer::new_for_http())
}