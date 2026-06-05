use axum::Router;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::app_state::AppState;
use crate::middleware::rate_limit::{auth_rate_limit, public_rate_limit};
use crate::{auth, feature};
use crate::openapi::ApiDoc;

pub fn build(app_state: AppState) -> Router {
    let public = Router::new()
        .merge(feature::router::public_router())
        .layer(public_rate_limit());

    let protected = Router::new()
        .merge(feature::router::protected_router())
        .layer(auth_rate_limit());

    let auth = auth::router().layer(auth_rate_limit());

    Router::new()
        .merge(public)
        .merge(protected)
        .merge(auth)
        .merge(
            SwaggerUi::new("/docs")
                .url("/api-docs/openapi.json", ApiDoc::openapi())
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state)
}