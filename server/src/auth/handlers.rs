use axum::{Json, extract::State};
use validator::Validate;
use super::{
    dto::{LoginRequest, RegisterRequest, TokenResponse}, 
    model::{RegisterUserCommand, LoginUserCommand},
    errors::AuthError,
};
use crate::app_state::AppState;

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 200, body = TokenResponse, description = "User registered successfully"),
        (status = 400, description = "Invalid request")
    )
)]
// Result is an enum that can be either Ok or Err
pub async fn register(
    State(app_state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<TokenResponse>, AuthError> {
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
    let token = app_state.auth_service.register_user(command).await?;

    Ok(Json(TokenResponse { token }))
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
    State(app_state): State<AppState>,
    Json(req): Json<LoginRequest>
) -> Result<Json<TokenResponse>, AuthError> {
    // don't worry about validating login request for now
    let command = LoginUserCommand {
        username: req.username,
        password: req.password,
    };

    let token = app_state.auth_service.login_user(command).await?;
    Ok(Json(TokenResponse { token }))
}