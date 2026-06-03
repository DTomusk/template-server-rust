use axum::Json;
use validator::Validate;
use super::{
    dto::{LoginRequest, RegisterRequest}, 
    model::RegisterUserCommand, 
    service::AuthService,
    errors::AuthError,
};

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
pub async fn register(Json(req): Json<RegisterRequest>) -> Result<&'static str, AuthError> {
    // Validate request
    // validate returns ValidationErrors, we map that to a string for now
    // if no error, simply continue
    req.validate().map_err(|e| AuthError::ValidationError(e.to_string()))?;

    // Construct command
    let command = RegisterUserCommand {
        username: req.username,
        password: req.password,
    };

    // Service has no state at the moment, so we can just instantiate here
    let service = AuthService;

    // Call service method
    // ? says to return early if error
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