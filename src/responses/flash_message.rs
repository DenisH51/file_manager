

use super::errors::session_err::SessionErrorType;
use super::success::general_success::SuccessesType;


#[derive(Debug)]
pub enum FlashKind {
    Error,
    Success,
}


#[derive(Debug)]
pub enum FlashCode {
    SessionErrors(SessionErrorType),
    Success(SuccessesType),
}


#[derive(Debug)]
pub struct FlashMessage {
    pub code: FlashCode,
    pub kind: FlashKind,
    pub message: &'static str,
}



impl FlashMessage {
    pub fn css_class(&self) -> &'static str {
        match self.kind {
            FlashKind::Error => "flash-error",
            FlashKind::Success => "flash-success",
        }
    }
}



impl FlashCode {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "invalid_session" => Some(Self::SessionErrors(SessionErrorType::InvalidSession)),
            "database_error" => Some(Self::SessionErrors(SessionErrorType::Database)),
            "session_creation_error" => Some(Self::SessionErrors(SessionErrorType::SessionCreation)),

            "login_successful" => Some(Self::Success(SuccessesType::Login)),
            "register_successful" => Some(Self::Success(SuccessesType::Register)),
            "logout_successful" => Some(Self::Success(SuccessesType::Logout)),

            _ => None,
        }
    }
}