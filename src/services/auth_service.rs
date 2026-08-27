use sqlx::SqlitePool;

use crate::models::requests::{
    RegisterRequest,
    LoginRequest,
};

use crate::models::user::UserLogin;
use crate::services::validator_service::{self};

use super::password_service;

use crate::errors::auth_err;






pub async fn register(db: &SqlitePool, data: RegisterRequest) -> Result<i64, auth_err::AuthError> {
    //check email
    validator_service::validate_email_reg(&data.email)
        .map_err(auth_err::AuthError::Validation)?;

    let email_exists = email_exists(&data.email, db)
        .await
        .map_err(auth_err::AuthError::Database)?;



    
    if email_exists{
        return Err(auth_err::AuthError::EmailAlreadyExists);
    }





    //check username
    validator_service::validate_username_reg(&data.username)
        .map_err(auth_err::AuthError::Validation)?;




    let username_exists = username_exists(&data.username, db)
        .await
        .map_err(auth_err::AuthError::Database)?;




    if username_exists{
        return Err(auth_err::AuthError::UsernameAlreadyExists);
    }
    
    //check password
    validator_service::validate_password_reg(&data.password, &data.confirm_password)
        .map_err(auth_err::AuthError::Validation)?;



    let password_hash = password_service::hash_password(&data.password)
        .map_err(|error| {
            println!("Password hash error: {:?}", error);
            auth_err::AuthError::PasswordHashError
        })?;
    

    //save data in db
    let user_id = save_data_db(db, &data, &password_hash)
        .await
        .map_err(auth_err::AuthError::Database)?;

    Ok(user_id)
    
}




pub async fn login(db: &SqlitePool, data: LoginRequest) -> Result<i64, auth_err::AuthError>{

    //check email
    validator_service::validate_email_login(&data.email)
        .map_err(auth_err::AuthError::Validation)?;


    //validate password
    validator_service::validate_password_login(&data.password)
        .map_err(auth_err::AuthError::Validation)?;

    let user = find_user_login_data(db, &data.email)
        .await
        .map_err(auth_err::AuthError::Database)?;

    let user: UserLogin = match user{
        Some(user) => user,
        None => {
            return Err(auth_err::AuthError::InvalidCredentials);
        }
    };

    password_service::verify_password(&data.password, &user.password_hash)
        .map_err(|_| auth_err::AuthError::InvalidCredentials)?;
    Ok(user.id)
}










//----------------------------------------------------------------

 
async fn find_user_login_data(db: &SqlitePool, email: &str,) -> Result<Option<UserLogin>, sqlx::Error> {
    
    let user = sqlx::query_as!(
        UserLogin,
        "
        SELECT id, password_hash
        FROM users
        WHERE email = ?
        ",
        email
    )
    .fetch_optional(db)
    .await?;

    Ok(user)
}



async fn email_exists(email: &str, db: &SqlitePool) -> Result<bool, sqlx::Error>{
    
    let account= sqlx::query!(
        "
        SELECT id
        FROM users
        WHERE email = ?
        ", email
    )
    .fetch_optional(db)
    .await?;

    Ok(account.is_some())
}



async fn username_exists(username: &str, db: &SqlitePool) -> Result<bool, sqlx::Error>{
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
    data: &RegisterRequest,
    password_hash: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
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

    Ok(result.last_insert_rowid())
}

