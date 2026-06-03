use axum::{Json, extract::State};
use validator::Validate;
use super::{
    dto::{LoginRequest, RegisterRequest}, 
    model::RegisterUserCommand, 
    errors::AuthError,
};
use crate::app_state::AppState;

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, description = "User registered successfully"),
        (status = 400, description = "Invalid request")
    )
)]
// Result is an enum that can be either Ok or Err
pub async fn register(
    State(app_state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<&'static str, AuthError> {
    // Validate request
    // validate returns ValidationErrors, we map that to a string for now
    // if no error, simply continue
    req.validate().map_err(|e| AuthError::ValidationError(e.to_string()))?;

    // Construct command
    let command = RegisterUserCommand {
        username: req.username,
        password: req.password,
    };

    // Call service method
    // ? says to return early if error
    app_state.auth_service.register_user(command).await?;

    Ok("user registered")
}

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User logged in successfully"),
        (status = 400, description = "Invalid request")
    )
)]
pub async fn login(
    State(_app_state): State<AppState>,
    Json(_req): Json<LoginRequest>
) -> Result<&'static str, String> {
    Ok("login not implemented")
}