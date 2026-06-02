use axum::{routing::get, Router};

use super::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/", get(handlers::get_feature))
}