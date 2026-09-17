
use crate::responses::errors::{auth_err, session_err};

use crate::responses::redirects::success_response;
use crate::responses::redirects::{auth_response, session_response};
use crate::responses::success::general_success;


use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;






pub fn handle_auth_errors(error: auth_err::AuthError) -> impl IntoResponse {
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

pub fn handle_session_auth_errors(error: session_err::SessionError) -> impl IntoResponse{

    //logging 

    

    session_response::session_response(error)

    
}

pub fn handle_session_logout_errors(error: session_err::SessionError, jar: CookieJar) -> impl IntoResponse{

    //logging 
    //there will logging error


    //redirect to success
    success_response::success_logout_response(general_success::Successes::Logout, jar)

    
}