use argon2::{
    Argon2, 
    PasswordHasher, 
    PasswordVerifier, 
    password_hash::{
        SaltString, 
        rand_core::OsRng,
    },
};

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2.hash_password(password.as_bytes(), &salt)
        .expect("Failed to hash password")
        .to_string()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = argon2::PasswordHash::new(hash).expect("Failed to parse password hash");
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}