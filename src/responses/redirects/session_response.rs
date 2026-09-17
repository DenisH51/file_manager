use crate::responses::flash_message::FlashCode;
use axum::response::{IntoResponse, Response, Redirect};
use axum_extra::extract::CookieJar;
use crate::services::flash_service;
use crate::responses::errors::session_err;



pub fn session_response(error: session_err::SessionError) -> Response {
    
    //beacause i have SessionError::Database(sqlx::Error) i need to convert to another type that dont contain real error
    let flash_response: session_err::SessionErrorType = error.to_flash_response();
    
    let cookie = flash_service::create_flash_cookie(FlashCode::SessionErrors(flash_response));

    let jar = CookieJar::new().add(cookie);

    let response = (jar, Redirect::to("/login")).into_response();

    response
}

