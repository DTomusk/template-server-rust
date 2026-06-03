use axum::Json;
use super::dto::{LoginRequest, RegisterRequest};

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
pub async fn register(Json(_req): Json<RegisterRequest>) -> Json<&'static str> {
    Json("register not implemented")
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
pub async fn login(Json(_req): Json<LoginRequest>) -> Json<&'static str> {
    Json("login not implemented")
}