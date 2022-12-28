use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use argon2::password_hash::errors::Error;

pub fn hash_password(pw: &str) -> Result<String, Error> {
    let password = pw.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2.hash_password(password, &salt)?.to_string();
    Ok(password_hash)   
}

pub fn verify_password(password_test: &str, password_hash: &str) -> Result<bool, Error>{
    // // Verify password against PHC string.
    // // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
    // // `Argon2` instance.
    let parsed_hash = PasswordHash::new(&password_hash)?;
    Ok(Argon2::default().verify_password(password_test.as_bytes(), &parsed_hash).is_ok())
}

