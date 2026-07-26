//account
pub struct Account {

    pub id: i64,

    pub email: String,

    pub password_hash: String,

    pub created_at: String,

    pub is_active: bool,

}


//profile
pub struct Profile {

    pub username: String,

    pub first_name: Option<String>,
    pub last_name: Option<String>,

    pub avatar_path: Option<String>,

}


//storage
pub struct Storage {

    pub used_bytes: i64,

    pub limit_bytes: i64,

    pub files_count: i64,

    pub folders_count: i64,
}


//secuirity
pub struct Security {

    pub last_login: Option<String>,

    pub two_factor_enabled: bool,

}


//main user
pub struct User{
    pub account: Account,

    pub profile: Profile,

    pub storage: Storage,

    pub security: Security,

    
}