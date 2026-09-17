use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use axum_extra::extract::cookie::CookieJar;

use sqlx::SqlitePool;

use crate::services::{session_service, error_service};
use crate::responses::errors::session_err;

//checks session to allow access to secure pages 
pub async fn require_auth(
    State(db): State<SqlitePool>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Response {

    let cookie = match jar.get("session_token") {

        Some(cookie) => cookie,

        None => {
            return Redirect::to("/login")
                .into_response();
        }
    };

    let session_token = cookie.value();

    match session_service::find_session(&db, session_token,).await{
        //Session valid
        Ok(Some(_user_id)) => {
            next.run(request).await
        }

        //session expired
        Ok(None) => {
            error_service::handle_session_auth_errors(session_err::SessionError::InvalidSession).into_response()
        }

        //Session Error
        Err(error) => {
            error_service::handle_session_auth_errors(error).into_response()
        }
    }
}