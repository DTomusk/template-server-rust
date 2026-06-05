use axum::{routing::get, Router};

use crate::app_state::AppState;

use super::handlers;

pub fn public_router() -> Router<AppState> {
    Router::new()
        .route("/feature", get(handlers::get_feature))
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .route("/feature/protected", get(handlers::get_protected_feature))
}