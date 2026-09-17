use axum::{
    extract::State,
    response::{IntoResponse},
    Form
};

use axum_extra::extract::CookieJar;
use crate::services::{error_service, session_service, success_service};
use crate::{services::auth_service};
use crate::responses::success::general_success::Successes;
use crate::responses::errors::{session_err};

use sqlx::SqlitePool;

use crate::models;


/* clean table user

async fn clear_users(db: &SqlitePool) -> Result<(), sqlx::Error> {
    let mut tx = db.begin().await?;

    sqlx::query!("DELETE FROM users")
        .execute(&mut *tx)
        .await?;

    sqlx::query!(
        "DELETE FROM sqlite_sequence WHERE name = 'users'"
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(())
}*/
//let _ = clear_users(&db).await;



pub async fn register(
    State(db): State<SqlitePool>,
    Form(data): Form<models::requests::RegisterRequest>,
) -> impl IntoResponse {

    


    match auth_service::register(&db, data).await {

        Ok(user_id) => {

            match session_service::create_session(&db, user_id).await {
                Ok(session_token) => {
                    
                    //Registration successful
                    //create cookie proceed directly when response creating in responses/redirect/success_response
                    success_service::handle_auth_success(Successes::Register, &session_token)
                        .into_response()
                }

                Err(error) => {
                   error_service::handle_session_auth_errors(error)
                        .into_response()
                }
            }
        }

        Err(error) => {
            error_service::handle_auth_errors(error).into_response()
        }
    }
}


pub async fn login(
    State(db): State<SqlitePool>,
    Form(data): Form<models::requests::LoginRequest>,
) -> impl IntoResponse {

    match auth_service::login(&db, data).await {

        Ok(user_id) => {
            

            match session_service::create_session(&db, user_id).await {
                Ok(session_token) => {
                    //Login successful
                    //create cookie proceed directly when response creating in responses/redirect/success_response
                    success_service::handle_auth_success(Successes::Login, &session_token)
                        .into_response()
                }

                Err(error) => {
                    error_service::handle_session_auth_errors(error).into_response()
                }
            }
        }


        Err(error) => {
            error_service::handle_auth_errors(error).into_response()
        }
    }
}





pub async fn logout(
    State(db): State<SqlitePool>,
    jar: CookieJar,
) -> impl IntoResponse {

    let session_token = match jar.get("session_token"){
        Some(cookie) => cookie.value().to_owned(),
        None => return error_service::handle_session_logout_errors(session_err::SessionError::NoSessionToken, jar).into_response(),
    };

    match session_service::delete_session(&db, &session_token).await{
        Ok(_) => success_service::handle_logout_success(Successes::Logout, jar).into_response(),

        Err(error) => error_service::handle_session_logout_errors(error, jar).into_response(),
    }
    
}