use argon2::{Argon2, PasswordHasher};
use argon2::password_hash::{SaltString, PasswordHash, PasswordVerifier};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Generate a random salt
    let salt = SaltString::from_b64("c29tZV9yYW5kb21fc2FsdA").unwrap(); // Base64 string

    // Create Argon2 hasher with default parameters
    let argon2 = Argon2::default();

    // Hash the password using the salt
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed)
        .expect("Failed to parse password hash");
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
