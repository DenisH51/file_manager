use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest{
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}