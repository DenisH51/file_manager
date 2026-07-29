use argon2::{
    Argon2,
    PasswordHasher,
};

use argon2::password_hash::{
    SaltString,
};


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