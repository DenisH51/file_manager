use sqlx::PgPool;
use uuid::Uuid;
use cookie::Cookie;


use crate::responses::errors::session_err::{SessionError};





 
pub async fn create_session(db: &PgPool, user_id: i64) -> Result<String, SessionError> {
    let session_token = Uuid::new_v4().to_string();

    sqlx::query!(
        "
            INSERT INTO sessions (
                user_id,
                session_token,
                expires_at
            )
            VALUES (
                $1,
                $2,
                CURRENT_TIMESTAMP + INTERVAL '1 minute'
            )
        ",
        user_id,
        session_token
    )
    .execute(db)
    .await
    .map_err(SessionError::SessionCreation)?;

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




pub async fn find_session(db: &PgPool, session_token: &str) -> Result<Option<i64>, SessionError> {
    let session = sqlx::query!(
        "
            SELECT user_id
            FROM sessions
            WHERE session_token = $1
            AND expires_at > CURRENT_TIMESTAMP
        ",
        session_token
    )
    .fetch_optional(db)
    .await
    .map_err(SessionError::Database)?;

    Ok(session.map(|session| session.user_id))
}



pub async fn delete_session(db: &PgPool, session_token: &str,) -> Result<(), SessionError> {
    sqlx::query!(
        "
            DELETE FROM sessions
            WHERE session_token = $1
        ",
        session_token
    )
    .execute(db)
    .await
    .map_err(SessionError::DeleteSession)?;

    Ok(())
}


