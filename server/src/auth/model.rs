use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct RegisterUserCommand {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}