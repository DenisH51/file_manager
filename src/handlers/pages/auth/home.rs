use axum::{
    extract::State,
    response::{Html, IntoResponse, Redirect},
};
use axum::http::StatusCode;

use axum_extra::extract::CookieJar;
use sqlx::SqlitePool;

use crate::{
    services::session_service,
    templates,
};





pub async fn auth_home(
    State(db): State<SqlitePool>,
    jar: CookieJar,
) -> impl IntoResponse  {


    //Get session cookie
    let cookie = match jar.get("session_token") {

        Some(cookie) => cookie,

        None => {
            return Redirect::to("/login")
                .into_response();
        }
    };

    //Get token
    let session_token = cookie.value();


    //check session
    match session_service::find_session(&db, session_token).await {

        //Session Exists
        Ok(Some(_user_id)) => {
            /*println!(
                "Authenticated user: {}",
                user_id
            );*/

            Html(
                templates::AUTH_HOME.to_string()
            )
            .into_response()
        },

        //session not exists or expire
        Ok(None) => {
            Redirect::to("/login").into_response()
        }


        //Error
        Err(error) => {

            println!(
                "Session verification error: {:?}",
                error
            );

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to verify session",
            )
                .into_response()
        }
    }



}