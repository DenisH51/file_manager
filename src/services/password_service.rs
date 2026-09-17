use argon2::{
    Argon2,
    PasswordHasher,
    PasswordHash,
    PasswordVerifier,
};

use argon2::password_hash::{
    SaltString,
};

use crate::responses::errors::auth_err::PasswordVerification;


use rand_core::OsRng;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error>{
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let hash = argon2
        .hash_password(
            password.as_bytes(),
            &salt
        )?
        .to_string();
    
    Ok(hash)
}

pub fn verify_password(
    password: &str,
    password_hash: &str,
) -> Result<(), PasswordVerification> {

   let parsed_hash = PasswordHash::new(password_hash)
        .map_err(PasswordVerification::Error)?;

    match Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
    {
        Ok(_) => Ok(()),

        Err(argon2::password_hash::Error::Password) => {
            Err(PasswordVerification::InvalidPassword)
        }

        Err(error) => {
            Err(PasswordVerification::Error(error))
        }
    }
}