use super::{
    errors::AuthError, 
    model::RegisterUserCommand, 
    password::hash_password,
};
use crate::repos::user_repo::UserRepo;
use crate::user::model::User;

pub struct AuthService {
    pub user_repo: UserRepo,
}

impl AuthService {
    pub fn new(user_repo: UserRepo) -> Self {
        Self { user_repo }
    }
    pub async fn register_user(
        &self,
        command: RegisterUserCommand,
    ) -> Result<(), AuthError> {
        // auth hashes password, checks if username is in use, persists user and generates token
        let existing_user = self.user_repo
            .get_user_by_username(&command.username)
            .await
            .map_err(|e| AuthError::RepositoryError(e))?;
        if existing_user.is_some() {
            return Err(AuthError::UserAlreadyExists);
        }
        let user = User::new(
            command.username.clone(), 
            hash_password(&command.password),
        );
        self.user_repo
            .create_user(&user)
            .await
            .map_err(|e| AuthError::RepositoryError(e))?;
        println!("Registering user: {:?}", command);
        Ok(())
    }
}