use axum::Json;
use super::dto::{LoginRequest, RegisterRequest};

pub async fn register(Json(_req): Json<RegisterRequest>) -> Json<&'static str> {
    Json("register not implemented")
}

pub async fn login(Json(_req): Json<LoginRequest>) -> Json<&'static str> {
    Json("login not implemented")
}