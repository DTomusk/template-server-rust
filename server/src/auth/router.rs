use axum::{routing::get, Router};

use super::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/register", get(handlers::register))
        .route("/login", get(handlers::login))
}