pub struct File {

    pub id: i64,

    pub user_id: i64,
    
    pub folder_id: Option<i64>,

    pub name: String,

    pub storage_path: String,

    pub size_bytes: i64,

    pub mime_type: String,

    pub created_at: String,

    pub is_deleted: bool,

}