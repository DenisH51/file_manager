#[derive(Debug)]
pub enum SessionError{
    Database(sqlx::Error),
    InvalidSession,
}


impl From<sqlx::Error> for SessionError {
    fn from(error: sqlx::Error) -> Self {
        SessionError::Database(error)
    }
}
