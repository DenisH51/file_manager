CREATE TABLE files (

    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id INTEGER NOT NULL,

    folder_id INTEGER,

    name TEXT NOT NULL,

    storage_path TEXT NOT NULL,

    size_bytes INTEGER NOT NULL,

    mime_type TEXT NOT NULL,

    created_at TEXT NOT NULL,

    is_deleted INTEGER NOT NULL DEFAULT 0,


    FOREIGN KEY(user_id)
    REFERENCES users(id)
    ON DELETE CASCADE,


    FOREIGN KEY(folder_id)
    REFERENCES folders(id)
    ON DELETE CASCADE

);