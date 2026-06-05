use utoipa::OpenApi;
use crate::auth::dto::{LoginRequest, RegisterRequest, TokenResponse};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::auth::handlers::register,
        crate::auth::handlers::login
    ),
    components(
        schemas(LoginRequest, RegisterRequest, TokenResponse)
    ),
    tags(
        (name = "auth", description = "Authentication related endpoints")
    )
)]
pub struct ApiDoc;