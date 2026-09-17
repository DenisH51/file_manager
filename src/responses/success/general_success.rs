
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
