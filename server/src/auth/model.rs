use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RegisterUserCommand {
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct LoginUserCommand {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// id here matches Claims sub type, consider making uuid
pub struct AuthUser {
    pub id: String,
}