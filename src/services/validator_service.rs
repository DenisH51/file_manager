
use crate::errors::auth_err::ValidationError;

pub fn validate_email_reg(email: &str) -> Result<(), ValidationError> {

    if email.trim().is_empty() {
        return Err(ValidationError::EmptyEmail);
    }

    if email.len() > 255 {
        return Err(ValidationError::EmailTooLong);
    }

    if !email.contains('@') {
        return Err(ValidationError::InvalidEmail);
    }

    Ok(())
}



pub fn validate_username_reg(username: &str) -> Result<(), ValidationError> {

    if username.trim().is_empty() {
        return Err(ValidationError::EmptyUsername);
    }

    if username.len() < 3 {
        return Err(ValidationError::UsernameTooShort);
    }

    if username.len() > 30 {
        return Err(ValidationError::UsernameTooLong);
    }

    if !username
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '_')
    {
        return Err(ValidationError::InvalidUsername);
    }

    Ok(())
}

pub fn validate_password_reg(password: &str, confirm_password: &str) -> Result<(), ValidationError> {
    println!("PASSWORD VALIDATOR: {:?}", password);

    if password.trim().is_empty() {
        return Err(ValidationError::EmptyPassword);
    }
    

    if password.len() < 8 {
        return Err(ValidationError::PasswordTooShort);
    }

    if password.len() > 64 {
        return Err(ValidationError::PasswordTooLong);
    }

    let forbidden = [
        "password",
        "12345678",
        "qwerty",
        "admin123",
    ];

    if forbidden.contains(&password) {
        return Err(ValidationError::WeakPassword);
    }
    
    if password != confirm_password{
        return Err(ValidationError::PasswordsDoNotMatch)
    }

    Ok(())
}


pub fn validate_email_login(email: &str) -> Result<(), ValidationError>{
    if email.trim().is_empty(){
        return Err(ValidationError::EmptyUsername);
    }

    if email.len() > 64 {
        return Err(ValidationError::UsernameTooLong);
    }

    Ok(())
}

pub fn validate_password_login(password: &str) -> Result<(), ValidationError>{
    if password.trim().is_empty() {
        return Err(ValidationError::EmptyPassword);
    }


    if password.len() > 127{
        return Err(ValidationError::PasswordTooLong);
    }

    Ok(())
}



