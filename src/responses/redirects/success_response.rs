use crate::responses::{flash_message::FlashCode, success::general_success};
use axum::response::{IntoResponse, Response, Redirect};
use axum_extra::extract::CookieJar;
use crate::services::flash_service;
use crate::services::session_service;



pub fn success_auth_response(success: general_success::Successes, session_token: &str) -> Response{

    let flash_response: general_success::SuccessesType = success.to_flash_response();

    let flash_cookie = flash_service::create_flash_cookie(FlashCode::Success(flash_response));
    let session_cookie = session_service::create_session_cookie(&session_token);

    let jar = CookieJar::new()
        .add(flash_cookie)
        .add(session_cookie);

    let response = (jar, Redirect::to("/auth/home")).into_response();

    response
}

pub fn success_logout_response(success: general_success::Successes, jar: CookieJar) -> Response{
        let flash_response: general_success::SuccessesType = success.to_flash_response();

    let flash_cookie = flash_service::create_flash_cookie(FlashCode::Success(flash_response));

    let jar = jar
        .add(flash_cookie)
        .remove("session_token");

    (jar, Redirect::to("/")).into_response()
}
