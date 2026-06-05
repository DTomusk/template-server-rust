use crate::auth::service::AuthService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    // use Arc for thread safety and shared ownership across handlers
    // means if you clone state, you still have the same AuthService instance
    pub auth_service: Arc<AuthService>,
}