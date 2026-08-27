use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};

use axum_extra::extract::cookie::CookieJar;

use sqlx::SqlitePool;

use crate::services::session_service;


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

    match session_service::find_session(
        &db,
        session_token,
    )
    .await
    {

        Ok(Some(_user_id)) => {

            next.run(request).await
        }

        Ok(None) => {

            Redirect::to("/login")
                .into_response()
        }

        Err(error) => {

            println!(
                "Session verification error: {:?}",
                error
            );

            Redirect::to("/login")
                .into_response()
        }
    }
}