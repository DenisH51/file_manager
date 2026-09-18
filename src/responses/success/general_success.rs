use crate::services::logging_service::{ToLog, LogLevel, LogData};


//for response
#[derive(Debug)]
pub enum SuccessesType{
    Login,
    Register,
    Logout,
}

impl SuccessesType{
    pub fn flash_code(&self) -> &'static str{
        match self{
            SuccessesType::Login => "login_successful",
            SuccessesType::Register => "register_successful",
            SuccessesType::Logout => "logout_successful"
        }
    }
    pub fn flash_message(&self) -> &'static str{
        match self{
            SuccessesType::Login => "Login successful.",
            SuccessesType::Register => "Account created successfully.",
            SuccessesType::Logout => "You have been logged out.",
        }
    }
}




#[derive(Debug)]
pub enum Successes{
    Login,
    Register,
    Logout,
}

impl Successes{
    pub fn to_flash_response(&self) -> SuccessesType{
        match self {
            Successes::Login => SuccessesType::Login,
            Successes::Register => SuccessesType::Register,
            Successes::Logout => SuccessesType::Logout,
        }
    }
}




impl ToLog for Successes{

    fn event_type(&self) -> String{
        format!("Successes::{:?}", self).to_owned()
    }


    fn log_data(&self) -> LogData {
        match self{
            Successes::Login => LogData { 
                level: LogLevel::Info,
                code: "login_successful",
                message: "User login successful",
                error: None
            },

            Successes::Register => LogData { 
                level: LogLevel::Info,
                code: "register_successful",
                message: "User registration successful",
                error: None
            },

            Successes::Logout => LogData { 
                level: LogLevel::Info,
                code: "logout_successful",
                message: "User logout successful",
                error: None
            },
        }
    }
}
