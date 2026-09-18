use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use crate::responses::success::general_success;
use crate::responses::redirects::success_response;
use crate::services;



pub fn handle_auth_success(success: general_success::Successes, session_token: &str, user_id: Option<i64>) -> impl IntoResponse{

    //logging
    services::logging_service::tracing_log(&success, user_id);


    success_response::success_auth_response(success, session_token)

    
}


pub fn handle_logout_success(success: general_success::Successes, jar: CookieJar, user_id: Option<i64>) -> impl IntoResponse{

    //logging
    services::logging_service::tracing_log(&success, user_id);

    success_response::success_logout_response(success, jar)
    
}