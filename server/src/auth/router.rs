use axum::{routing::post, Router};

use crate::app_state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
}