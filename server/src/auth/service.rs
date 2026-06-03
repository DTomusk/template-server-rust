use super::errors::AuthError;

use super::model::RegisterUserCommand;

pub struct AuthService;

impl AuthService {
    pub async fn register_user(
        &self,
        command: RegisterUserCommand,
    ) -> Result<(), AuthError> {
        println!("Registering user: {:?}", command);
        Ok(())
    }
}