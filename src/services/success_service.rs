use axum::response::IntoResponse;
use axum_extra::extract::CookieJar;
use crate::responses::success::general_success;
use crate::responses::redirects::success_response;



pub fn handle_auth_success(success: general_success::Successes, session_token: &str) -> impl IntoResponse{

    success_response::success_auth_response(success, session_token)

    
}


pub fn handle_logout_success(success: general_success::Successes, jar: CookieJar) -> impl IntoResponse{

    success_response::success_logout_response(success, jar)
    
}