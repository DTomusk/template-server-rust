use axum::{
    extract::FromRequestParts,
    http::request::Parts,
};
use tower_governor::{GovernorError, key_extractor::KeyExtractor};

use super::{
    errors::AuthError,
    model::AuthUser,
};

use crate::{app_state::AppState};

// If a handler method includes AuthUser
// then this will run to extract it 
// Note: ensure auth middleware is run before extractor
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AuthError;

    async fn from_request_parts(
        req: &mut Parts, 
        _state: &AppState
    ) -> Result<Self, Self::Rejection>
    {
        req.extensions
            .get::<AuthUser>()
            .cloned()
            .ok_or(AuthError::Unauthorized)
    }
}

#[derive(Clone)]
pub struct UserIdExtractor;

impl KeyExtractor for UserIdExtractor { 
    type Key = String;
    
    fn extract<T>(&self, req: &axum::http::Request<T>) -> Result<String, GovernorError> {
        req.extensions()
            .get::<AuthUser>()
            .map(|u| u.id.to_string())
            .ok_or_else(|| GovernorError::UnableToExtractKey)
    }
}