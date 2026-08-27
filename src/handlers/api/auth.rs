use axum::{
    extract::State,
    http::{header::{SET_COOKIE, LOCATION}, StatusCode},
    response::{IntoResponse, Json, Redirect},
    Form,
    extract::ConnectInfo,
};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::CookieJar;

use auth_err::AuthError;
use crate::services::session_service;
use crate::{services::auth_service};

use crate::errors::{self, auth_err};

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
}
*/
//let _ = clear_users(&db).await;


pub async fn register(
    State(db): State<SqlitePool>,
    Form(data): Form<models::requests::RegisterRequest>,
) -> impl IntoResponse {

    


    match auth_service::register(&db, data).await {

        Ok(user_id) => {
            println!("User registered");

            match session_service::create_session(&db, user_id).await {
                Ok(session_token) => {
                    
                    let cookie = session_service::create_session_cookie(&session_token);

                    
                    (
                        StatusCode::OK,
                        [(SET_COOKIE, cookie.to_string())],
                        Json(models::api_error::ApiErrorResponse {
                            field: "general",
                            message: "Registration successful",
                        }),
                    )
                        .into_response()
                }

                Err(error) => {
                    println!("Session creation error: {:?}", error);

                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(models::api_error::ApiErrorResponse {
                            field: "general",
                            message: "Unable to create session",
                        }),
                    )
                        .into_response()
                }
            }
        }





        Err(AuthError::Validation(error)) => {
            let response = models::api_error::ApiErrorResponse{
                field: error.field(),
                message: error.message(),
            };

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(response),
            )
            .into_response()
        }

        Err(AuthError::EmailAlreadyExists) => {
            let response = models::api_error::ApiErrorResponse{
                field: "email",
                message: "Email already registered",
            };
            (
               StatusCode::UNAUTHORIZED,
               Json(response),
            )
            .into_response()
        }

        Err(AuthError::UsernameAlreadyExists) => {
            let response = models::api_error::ApiErrorResponse{
                field: "username",
                message: "Username is not available",
            };

            (
                StatusCode::UNAUTHORIZED,
                Json(response),
            )
            .into_response()
        }

        Err(AuthError::Database(error)) => {
            println!("Database error: {}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(models::api_error::ApiErrorResponse {
                    field: "general",
                    message: "Something went wrong on our side. Please try again later.",
                }),
            )
            .into_response()
        }

        Err(AuthError::PasswordHashError) => {
            println!("Password hash error");

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(models::api_error::ApiErrorResponse {
                    field: "general",
                    message: "Unable to create your account. Please try again later.",
                }),
            )
            .into_response()
        }

        Err(error) => {
            println!("Auth error: {:?}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(models::api_error::ApiErrorResponse {
                    field: "general",
                    message: "Something went wrong on our side. Please try again later.",
                }),
            )
            .into_response()
        }
    }
}


pub async fn login(
    State(db): State<SqlitePool>,
    Form(data): Form<models::requests::LoginRequest>,
) -> impl IntoResponse {

    match auth_service::login(&db, data).await {

        Ok(user_id) => {
            println!("User logged in");

            match session_service::create_session(&db, user_id).await {
                Ok(session_token) => {
                    
                    let cookie = session_service::create_session_cookie(&session_token);

                    
                    (
                        StatusCode::OK,
                        [(SET_COOKIE, cookie.to_string())],
                        Json(models::api_error::ApiErrorResponse {
                            field: "general",
                            message: "Login successful",
                        }),
                    )
                        .into_response()
                }

                Err(error) => {
                    println!("Session creation error: {:?}", error);

                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(models::api_error::ApiErrorResponse {
                            field: "general",
                            message: "Unable to create session",
                        }),
                    )
                        .into_response()
                }
            }
        }





        Err(AuthError::Validation(error)) => {
            let response = models::api_error::ApiErrorResponse{
                field: error.field(),
                message: error.message(),
            };

            (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(response),
            )
            .into_response()
        }

        Err(AuthError::InvalidCredentials) => {
            let response = models::api_error::ApiErrorResponse{
                field: "general",
                message: "Invalid email or password",
            };
            (
                StatusCode::UNAUTHORIZED,
                Json(response),
            )
            .into_response()
        }

        Err(AuthError::Database(error)) => {
            println!("Database error: {}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(models::api_error::ApiErrorResponse {
                    field: "general",
                    message: "Something went wrong on our side. Please try again later.",
                }),
            )
            .into_response()
        }

        Err(error) => {
            println!("Auth error: {:?}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(models::api_error::ApiErrorResponse {
                    field: "general",
                    message: "Something went wrong on our side. Please try again later.",
                }),
            )
            .into_response()
        }
    }
}





pub async fn logout(
    State(db): State<SqlitePool>,
    jar: CookieJar,
) -> impl IntoResponse {

    let cookie = match jar.get("session_token") {

        Some(cookie) => cookie,

        None => {
            return Redirect::to("/login")
                .into_response();
        }
    };

    let session_token = cookie.value().to_owned();

    match session_service::delete_session(
        &db,
        &session_token,
    ).await {

        Ok(_) => {

            let remove_cookie = Cookie::build(("session_token", ""))
                .path("/")
                .build();

            let jar = jar.remove(remove_cookie);

            (
                jar,
                Redirect::to("/login"),
            )
                .into_response()
        }

        Err(error) => {

            println!(
                "Logout session error: {:?}",
                error
            );

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(models::api_error::ApiErrorResponse {
                    field: "general",
                    message: "Unable to logout",
                }),
            )
                .into_response()
        }
    }
}