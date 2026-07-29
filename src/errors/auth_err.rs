

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
    WeakPassword
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
        }
    }
}




#[derive(Debug)]
pub enum AuthError {
    Validation(ValidationError),
    
    EmailAlreadyExists,

    UsernameAlreadyExists,

    PasswordHashError,
    

    Database(sqlx::Error),
}

