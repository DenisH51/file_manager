use crate::services::logging_service::{LogData, LogLevel, ToLog};

//for http response
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

impl ToLog for SessionError {

    fn event_type(&self) -> String{
        format!("SessionError::{:?}", self).to_owned()
    }


    fn log_data(&self) -> LogData {
        match self{
            SessionError::InvalidSession => LogData{
                level: LogLevel::Warn,
                code: "invalid_session",
                message: "The user attempted to access a resource using an invalid or expired session",
                error: None
            },


            SessionError::Database(err) => LogData{
                level: LogLevel::Error,
                code: "database_error",
                message: "The application failed to retrieve session data from the database",
                error: Some(err.to_string())
            },

            SessionError::SessionCreation(err) => LogData{
                level: LogLevel::Error,
                code: "session_creation_error",
                message: "The application failed to create a new session for the user",
                error: Some(err.to_string())
            },

            SessionError::DeleteSession(err) => LogData{
                level: LogLevel::Error,
                code: "delete_Session_error",
                message: "The application failed to delete the user's session",
                error: Some(err.to_string())
            },

            SessionError::NoSessionToken => LogData{
                level: LogLevel::Warn,
                code: "no_session_token",
                message: "The user attempted to authenticate or log out without providing a session token",
                error: None
            },
        }
    }
}
