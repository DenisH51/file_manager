
#[derive(Debug)]
pub enum SessionErrorType{
    Database,
    SessionCreation,
    InvalidSession,
}


impl SessionErrorType {
    pub fn flash_code(&self) -> &'static str {
        match self {
            SessionErrorType::InvalidSession => "invalid_session",
            SessionErrorType::Database => "database_error",
            SessionErrorType::SessionCreation => "session_creation_error",
        }
    }

    pub fn flash_message(&self) -> &'static str {
        match self {
            SessionErrorType::InvalidSession => "Session expired. Login again",
            SessionErrorType::Database => "Something went wrong. Please try again",
            SessionErrorType::SessionCreation => "Something went wrong. Please try again",
        }
    }
}


#[derive(Debug)]
pub enum SessionError{
    Database(sqlx::Error),
    SessionCreation(sqlx::Error),
    InvalidSession,

    DeleteSession(sqlx::Error),
    NoSessionToken,
}


impl SessionError {

    pub fn to_flash_response(&self) -> SessionErrorType {
        match self {
            SessionError::Database(_) => SessionErrorType::Database,
            SessionError::SessionCreation(_) => SessionErrorType::SessionCreation,
            SessionError::InvalidSession => SessionErrorType::InvalidSession,
            
            SessionError::NoSessionToken => unreachable!(),
            SessionError::DeleteSession(_) => unreachable!()
        }
    }
}

