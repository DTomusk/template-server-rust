use axum::{
    body::Body,
    http::{Request, Response},
    extract::State, middleware::Next,
};

use crate::{
    app_state::AppState,
    auth::model::AuthUser,
    auth::errors::AuthError,
};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>, 
    next: Next) -> Result<Response<Body>, AuthError> {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AuthError::Unauthorized)?;

    let claims = state
        .auth_service
        .verify_token(token)
        .map_err(|_| AuthError::Unauthorized)?;

    let user = AuthUser { id: claims.sub };

    req.extensions_mut().insert(user);

    Ok(next.run(req).await)
}