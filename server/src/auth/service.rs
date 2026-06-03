use super::{errors::AuthError, model::RegisterUserCommand};
use crate::repos::user_repo::UserRepo;

pub struct AuthService {
    pub user_repo: UserRepo,
}

impl AuthService {
    pub async fn register_user(
        &self,
        command: RegisterUserCommand,
    ) -> Result<(), AuthError> {
        // auth hashes password, checks if username is in use, persists user and generates token
        self.user_repo
            .create_user(&command.username, &command.password)
            .await
            .map_err(|e| AuthError::RepositoryError(e))?;
        println!("Registering user: {:?}", command);
        Ok(())
    }
}