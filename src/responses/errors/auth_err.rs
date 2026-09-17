




#[derive(Debug)]
pub enum ValidationError {
    EmptyEmail,
    EmailTooLong,
    InvalidEmail,

    UsernameTooShort,
    EmptyUsername,
    UsernameTooLong,
    InvalidUsername,
    
    EmptyPassword,
    PasswordTooShort,
    PasswordTooLong,
    WeakPassword,
    PasswordsDoNotMatch,
}

#[derive(Debug)]
pub enum PasswordVerification {
    InvalidPassword,
    Error(argon2::password_hash::Error),
}

#[derive(Debug)]
pub enum BusinessError {
    EmailAlreadyExists,
    UsernameAlreadyExists,
    InvalidCredentials,
}

#[derive(Debug)]
pub enum InfrastructureError{
    Database(sqlx::Error),
    PasswordVerificationError(argon2::password_hash::Error),
    PasswordHashError(argon2::password_hash::Error)
}

#[derive(Debug)]
pub enum AuthError{
    Validation(ValidationError),
    Business(BusinessError),
    Infrastructure(InfrastructureError),
}







impl ValidationError {
    pub fn message(&self) -> &'static str {
        match self {
            ValidationError::EmptyEmail => "Email is required",
            ValidationError::EmailTooLong => "Email is too long",
            ValidationError::InvalidEmail => "Invalid email format",
            
            ValidationError::UsernameTooShort => "Username must contain at least 3 characters",
            ValidationError::EmptyUsername => "Username is required",
            ValidationError::UsernameTooLong => "Username is too long",
            ValidationError::InvalidUsername => "Username contains invalid characters",

            ValidationError::EmptyPassword => "Password is required",
            ValidationError::PasswordTooShort => "Password must contain at least 8 characters",
            ValidationError::PasswordTooLong => "Password is too long",
            ValidationError::WeakPassword => "Choose a stronger password",

            ValidationError::PasswordsDoNotMatch => "Passwords do not match",
        }
    }


    pub fn field(&self) -> &'static str {
        match self {
            ValidationError::EmptyEmail
            | ValidationError::EmailTooLong
            | ValidationError::InvalidEmail => "email",

            ValidationError::EmptyUsername
            | ValidationError::UsernameTooLong
            | ValidationError::UsernameTooShort
            | ValidationError::InvalidUsername => "username",

            ValidationError::EmptyPassword
            | ValidationError::PasswordTooLong
            | ValidationError::PasswordTooShort
            | ValidationError::WeakPassword => "password",

            ValidationError::PasswordsDoNotMatch => "confirm_password",
        }
    }


}

impl BusinessError {
    pub fn message(&self) -> &'static str{
        match self{
            BusinessError::EmailAlreadyExists => "Email already exists",
            BusinessError::UsernameAlreadyExists => "Username already exists",
            BusinessError::InvalidCredentials => "Invalid Credentials",
        }
    }


    pub fn field(&self) -> &'static str{
        match self{
            BusinessError::EmailAlreadyExists => "email",
            BusinessError::UsernameAlreadyExists => "username",
            BusinessError::InvalidCredentials => "general"
        }
    }   
}


impl InfrastructureError {
    pub fn message(&self) -> &'static str{
        match self{
            InfrastructureError::Database(_) => "Somethig going wrong",
            InfrastructureError::PasswordHashError(_) => "Somethig going wrong",
            InfrastructureError::PasswordVerificationError(_) => "Somethig going wrong",
        }
    }

    pub fn field(&self) -> &'static str{
        match self{
            InfrastructureError::Database(_)
            |InfrastructureError::PasswordHashError(_)
            |InfrastructureError::PasswordVerificationError(_) => "general",
        }
    }
}
