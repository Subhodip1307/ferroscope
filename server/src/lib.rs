use argon2::password_hash::{PasswordHash, PasswordHasher, SaltString};
use argon2::{Algorithm, Argon2, Params, PasswordVerifier, Version};
use rand::rngs::OsRng;



fn argon2_instance() -> Argon2<'static> {
    let params = Params::new(
        19_456, // memory (KB)
        2,      // iterations
        1,      // parallelism
        None,
    )
    .unwrap();

    Argon2::new(Algorithm::Argon2id, Version::V0x13, params)
}

pub fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = argon2_instance();

    argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let argon2 = argon2_instance();

    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
