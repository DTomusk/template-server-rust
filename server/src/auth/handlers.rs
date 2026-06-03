use axum::Json;
use validator::Validate;
use super::dto::{LoginRequest, RegisterRequest};
use super::model::RegisterUserCommand;
use super::service::AuthService;

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
pub async fn register(Json(req): Json<RegisterRequest>) -> Result<&'static str, String> {
    // Validate request
    req.validate().map_err(|e| e.to_string())?;

    // Construct command
    let command = RegisterUserCommand {
        username: req.username,
        password: req.password,
    };

    // Service has no state at the moment, so we can just instantiate here
    let service = AuthService;

    // Call service method
    service.register_user(command).await?;

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
pub async fn login(Json(_req): Json<LoginRequest>) -> Result<&'static str, String> {
    Ok("login not implemented")
}