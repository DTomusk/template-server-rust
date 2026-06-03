pub struct UserRepo;

impl UserRepo {
    pub async fn create_user(
        &self,
        username: &str, 
        password: &str,
    ) -> Result<(), String> {
        println!("Creating user: {} with password: {}", username, password);
        Ok(())
    }
}