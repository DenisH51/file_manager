use serde::Deserialize;


#[derive(Debug)]
pub struct Session {
    pub user_id: i64,
    pub session_token: String,
}