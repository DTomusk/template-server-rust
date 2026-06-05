// We're technically leaking persistence details into the domain
// But we'll keep it for simplicity
// If domain and persistence models diverge, then we can separate out a UserRow in the repo layer
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub fn new(username: String, password_hash: String) -> Self {
        Self { 
            id: uuid::Uuid::new_v4(), 
            username, 
            password_hash,
        }
    }
}