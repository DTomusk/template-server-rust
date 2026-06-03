use super::model::RegisterUserCommand;

pub struct AuthService;

impl AuthService {
    pub async fn register_user(
        &self,
        command: RegisterUserCommand,
    ) -> Result<(), String> {
        println!("Registering user: {:?}", command);
        Ok(())
    }
}