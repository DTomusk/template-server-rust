use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};

use super::{
    errors::AuthError,
    model::AuthUser,
};

use crate::{app_state::AppState};

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection>
    {
        let token = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.strip_prefix("Bearer "))
            .ok_or(AuthError::InvalidCredentials)?;

        let claims = state
            .auth_service
            .verify_token(token)
            .map_err(|_| AuthError::Unauthorized)?;

        Ok(AuthUser { 
            id: claims.sub,
        })
    }
}