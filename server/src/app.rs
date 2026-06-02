use axum::Router;
use tower_http::trace::TraceLayer;
use crate::{auth, feature};

pub fn build() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/feature", feature::router())
        .layer(TraceLayer::new_for_http())
}