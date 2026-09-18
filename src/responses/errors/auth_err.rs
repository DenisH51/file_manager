
use crate::services::logging_service::{LogLevel, LogData, ToLog};


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



//for logging
impl ToLog for AuthError {

    fn event_type(&self) -> String{
        format!("AuthError::{:?}", self).to_owned()
    }

    fn log_data(&self) -> LogData {
        match self{
            AuthError::Validation(validation_error) => {
                match validation_error{
                    ValidationError::EmptyEmail => LogData{
                        level: LogLevel::Warn,
                        code: "empty_email",
                        message: "User submitted an empty email address",
                        error: None,
                    },

                    ValidationError::EmailTooLong => LogData{
                        level: LogLevel::Warn,
                        code: "User submitted an email address that exceeds the allowed length",
                        message: "",
                        error: None,
                    },

                    ValidationError::InvalidEmail => LogData{
                        level: LogLevel::Warn,
                        code: "invalid_email",
                        message: "User submitted an email address with an invalid format",
                        error: None,
                    },

                    ValidationError::UsernameTooShort => LogData{
                        level: LogLevel::Warn,
                        code: "username_too_short",
                        message: "User submitted a username shorter than the minimum allowed length",
                        error: None,
                    },

                    ValidationError::EmptyUsername => LogData{
                        level: LogLevel::Warn,
                        code: "empty_username",
                        message: "User submitted an empty username",
                        error: None,
                    },

                    ValidationError::UsernameTooLong => LogData{
                        level: LogLevel::Warn,
                        code: "username_too_long",
                        message: "User submitted a username that exceeds the allowed length",
                        error: None,
                    },

                    ValidationError::InvalidUsername => LogData{
                        level: LogLevel::Warn,
                        code: "invalid_username",
                        message: "User submitted a username with an invalid format",
                        error: None,
                    },

                    ValidationError::EmptyPassword => LogData{
                        level: LogLevel::Warn,
                        code: "empty_password",
                        message: "User submitted an empty password",
                        error: None,
                    },

                    ValidationError::PasswordTooShort => LogData{
                        level: LogLevel::Warn,
                        code: "password_too_short",
                        message: "User submitted a password shorter than the minimum allowed length",
                        error: None,
                    },

                    ValidationError::PasswordTooLong => LogData{
                        level: LogLevel::Warn,
                        code: "password_too_long",
                        message: "User submitted a password that exceeds the maximum allowed length",
                        error: None,
                    },

                    ValidationError::WeakPassword => LogData{
                        level: LogLevel::Warn,
                        code: "weak_password",
                        message: "User submitted a password that does not meet the required security rules",
                        error: None,
                    },

                    ValidationError::PasswordsDoNotMatch => LogData{
                        level: LogLevel::Warn,
                        code: "passwords_do_not_match",
                        message: "User submitted two passwords that do not match",
                        error: None,
                    },
                }
            },

            AuthError::Business(business_error) => {
                match business_error{
                    BusinessError::EmailAlreadyExists => LogData{
                        level: LogLevel::Warn,
                        code: "email_already_exists",
                        message: "User attempted to register with an email address that is already registered",
                        error: None,
                    },

                    BusinessError::UsernameAlreadyExists => LogData{
                        level: LogLevel::Warn,
                        code: "username_already_exists",
                        message: "User attempted to register with a username that is already in use",
                        error: None,
                    },

                    BusinessError::InvalidCredentials => LogData{
                        level: LogLevel::Warn,
                        code: "invalid_credentials",
                        message: "User attempted to log in with invalid credentials",
                        error: None,
                    },
                }
            },

            AuthError::Infrastructure(infrastructure_error) => {
                match infrastructure_error{
                    InfrastructureError::Database(err) => LogData{
                        level: LogLevel::Error,
                        code: "database_error",
                        message: "The application failed to perform a database operation",
                        error: Some(err.to_string()),
                    },

                    InfrastructureError::PasswordVerificationError(err) => LogData{
                        level: LogLevel::Error,
                        code: "password_verification_error",
                        message: "The application failed to verify the user's password",
                        error: Some(err.to_string()),
                    },

                    InfrastructureError::PasswordHashError(err) => LogData{
                        level: LogLevel::Error,
                        code: "password_hash_error",
                        message: "The application failed to hash the user's password",
                        error: Some(err.to_string()),
                    },
                }
            },
        }
    }
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
