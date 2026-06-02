use axum::Router;
use tower_http::trace::TraceLayer;
use crate::feature;

pub fn build() -> Router {
    Router::new()
        .nest("/feature", feature::router())
        .layer(TraceLayer::new_for_http())
}