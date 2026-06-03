use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::{auth, feature};
use crate::openapi::ApiDoc;

pub fn build() -> Router {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/feature", feature::router())
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(TraceLayer::new_for_http())
}