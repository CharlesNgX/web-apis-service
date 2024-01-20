use crate::models::User;
use crate::models::Credentials;

use argon2::{
    password_hash::{
        Error,
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use rand::{Rng, distributions::Alphanumeric};

pub fn authorize_user(user: &User, credentials: Credentials) -> Result<String, Error> {
    let argon2 = Argon2::default();
    let db_hash = PasswordHash::new(&user.password)?;
    argon2.verify_password(credentials.password.as_bytes(), &db_hash)?;

    let session_id = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(128)
        .map(char::from)
        .collect();

    Ok(session_id)
}

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(OsRng);
    let argon2 = Argon2::default();
    let hashed_password = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hashed_password.to_string())
}