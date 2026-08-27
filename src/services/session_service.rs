use sqlx::SqlitePool;
use uuid::Uuid;
use cookie::Cookie;


use crate::{errors::session_err::SessionError, models::session::Session};





 
pub async fn create_session(db: &SqlitePool, user_id: i64) -> Result<String, SessionError>{
    
    let session_token = Uuid::new_v4().to_string();


    sqlx::query!(
        "
        INSERT INTO sessions (
            user_id,
            session_token,
            expires_at
        )
        VALUES (
            ?,
            ?,
            datetime('now', '+7 days')
        )
        ",
        user_id,
        session_token
    )
    .execute(db)
    .await?;

    Ok(session_token)
}

pub fn create_session_cookie(
    session_token: &str,
) -> Cookie<'static> {
    Cookie::build(("session_token", session_token.to_owned()))
        .http_only(true)
        .secure(false)
        .path("/")
        .build()
}


pub async fn find_session(
    db: &SqlitePool,
    session_token: &str,
) -> Result<Option<i64>, SessionError> {

    let session = sqlx::query!(
        "
        SELECT user_id
        FROM sessions
        WHERE session_token = ?
        AND expires_at > datetime('now')
        ",
        session_token
    )
    .fetch_optional(db)
    .await?;

    Ok(session.map(|session| session.user_id))
}


pub async fn delete_session(
    db: &SqlitePool,
    session_token: &str,
) -> Result<(), SessionError> {

    sqlx::query!(
        "
        DELETE FROM sessions
        WHERE session_token = ?
        ",
        session_token
    )
    .execute(db)
    .await?;

    Ok(())
}

pub fn delete_session_cookie() -> Cookie<'static> {
    Cookie::build(("session_token", ""))
        .http_only(true)
        .secure(false)
        .path("/")
        .max_age(time::Duration::seconds(0))
        .build()
}

//pub async fn refresh_session()


