use axum::{routing::post, Router};

use crate::app_state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(handlers::register))
        .route("/auth/login", post(handlers::login))
}