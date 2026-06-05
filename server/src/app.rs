use axum::{Router, middleware};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::{
    app_state::AppState, 
    auth, 
    feature, 
    middleware::rate_limit::{
        auth_rate_limit, 
        public_rate_limit, 
        user_rate_limit
    }, 
    openapi::ApiDoc,
};

pub fn build(app_state: AppState) -> Router {
    let public = feature::router::public_router()
        .layer(public_rate_limit());

    let protected = feature::router::protected_router()
        .layer(user_rate_limit())
        .layer(middleware::from_fn_with_state(
            app_state.clone(), 
            auth::middleware::auth_middleware
        ));

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