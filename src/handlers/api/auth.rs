use axum::{
    extract::State,
    Form,
};

use crate::services::auth_service;

use crate::errors::auth_err;

use sqlx::SqlitePool;

use crate::models::requests::{
    RegistrateRequest,
    LoginRequest,
};



pub async fn registrate(
    State(db): State<SqlitePool>,
    Form(data): Form<RegistrateRequest>
    ){
    
    let result = auth_service::registrate(
        &db,
        data,
        )
        .await;

    match result{
        Ok(_) => {
            println!("User registrated")
        }

        Err(error) => {
            match error{

                auth_err::AuthError::Validation(validation) => {
                println!("{}", validation.message());
                }

                auth_err::AuthError::EmailAlreadyExists => {
                    println!("Email already registered");
                }

                auth_err::AuthError::UsernameAlreadyExists => {
                    println!("Username is not available");
                }

                auth_err::AuthError::Database(e) => {
                    println!("Database error: {}", e);
                }

                auth_err::AuthError::PasswordHashError => {
                    println!("Password Hash not exist");
                }
            }
        }
    }
        
    
    

}


pub async fn login(
    State(db): State<SqlitePool>,
    Form(data): Form<LoginRequest>
    ){
}


pub async fn logout(){

}

