use sqlx::SqlitePool;

use crate::models::requests::{
    RegistrateRequest,
    LoginRequest,
};
use crate::services::validator_service;

use super::password_service;

use crate::errors::auth_err;






pub async fn registrate(
    db: &SqlitePool,
    data: RegistrateRequest
    ) -> Result<(), auth_err::AuthError> {
    
    //check email
    
    validator_service::validate_email(&data.email)
        .map_err(auth_err::AuthError::Validation)?;

    let email_exists = check_email_db(&data.email, db)
        .await
        .map_err(auth_err::AuthError::Database)?;
    
    if email_exists{
        return Err(auth_err::AuthError::EmailAlreadyExists);
    }


    //check username
    validator_service::validate_username(&data.username)
        .map_err(auth_err::AuthError::Validation)?;

    let username_exists = check_username_db(&data.username, db)
        .await
        .map_err(auth_err::AuthError::Database)?;

    if username_exists{
        return Err(auth_err::AuthError::UsernameAlreadyExists);
    }

    
    //check password
    validator_service::validate_password(&data.password)
        .map_err(auth_err::AuthError::Validation)?;

    let password_hash = password_service::hash_password(&data.password)
        .map_err(|_| auth_err::AuthError::PasswordHashError)?;


    save_data_db(db, &data, &password_hash)
        .await
        .map_err(auth_err::AuthError::Database)?;


    Ok(())
    
}

pub async fn login_user(){

}

pub async fn logout_user(){

}









//----------------------------------------------------------------


async fn check_email_db(email: &str, db: &SqlitePool) -> Result<bool, sqlx::Error>{
    let account = sqlx::query!(
        "
        SELECT id
        FROM users
        WHERE email = ?
        ",
        email
    )
    .fetch_optional(db)
    .await?;

    Ok(account.is_some())
}


async fn check_username_db(username: &str, db: &SqlitePool) -> Result<bool, sqlx::Error>{
    let account = sqlx::query!(
        "
        SELECT id
        FROM users
        WHERE username = ?
        ",
        username
    )
        .fetch_optional(db)
        .await?;

    Ok(account.is_some())
}






async fn save_data_db(
    db: &SqlitePool,
    data: &RegistrateRequest,
    password_hash: &str,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        "
        INSERT INTO users (
            email,
            username,
            password_hash
        )
        VALUES (?, ?, ?);
        ",
        data.email,
        data.username,
        password_hash,
    )
    .execute(db)
    .await?;

    Ok(())
}

