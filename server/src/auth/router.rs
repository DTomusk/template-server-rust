use axum::{routing::post, Router};

use super::handlers;

pub fn router() -> Router {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
}