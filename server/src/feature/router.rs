use axum::{routing::get, Router};

use crate::app_state::AppState;

use super::handlers;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handlers::get_feature))
}