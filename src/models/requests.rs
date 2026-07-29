use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrateRequest{
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}