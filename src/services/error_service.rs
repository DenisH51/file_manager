
use crate::responses::errors::{auth_err, session_err};

use crate::responses::redirects::success_response;
use crate::responses::redirects::{auth_response, session_response};
use crate::responses::success::general_success;
use crate::services::logging_service;

use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;






pub fn handle_auth_errors(error: auth_err::AuthError, user_id: Option<i64>) -> impl IntoResponse {
    //logging 
    logging_service::tracing_log(&error, user_id);
    
    match error {
        auth_err::AuthError::Validation(err) => {
            auth_response::validation_response(err)
        }

        auth_err::AuthError::Business(err) => {
            auth_response::business_response(err)
        }

        auth_err::AuthError::Infrastructure(err) => {
            auth_response::infrastructure_response(err)
        } 
    }
}

pub fn handle_session_auth_errors(error: session_err::SessionError, user_id: Option<i64>) -> impl IntoResponse{

    //logging 
    logging_service::tracing_log(&error, user_id);
    

    session_response::session_response(error)

    
}

pub fn handle_session_logout_errors(error: session_err::SessionError, jar: CookieJar, user_id: Option<i64>) -> impl IntoResponse{

    //logging 
    logging_service::tracing_log(&error, user_id);

    //redirect to success
    success_response::success_logout_response(general_success::Successes::Logout, jar)

    
}