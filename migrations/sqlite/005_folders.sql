CREATE TABLE folders (

    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id INTEGER NOT NULL,

    parent_id INTEGER,

    name TEXT NOT NULL,

    created_at TEXT NOT NULL,


    FOREIGN KEY(user_id)
    REFERENCES users(id)
    ON DELETE CASCADE,


    FOREIGN KEY(parent_id)
    REFERENCES folders(id)
    ON DELETE CASCADE

);