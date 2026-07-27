CREATE TABLE security (

    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id INTEGER NOT NULL UNIQUE,

    last_login TEXT,

    two_factor_enabled INTEGER NOT NULL DEFAULT 0,


    FOREIGN KEY(user_id)
    REFERENCES users(id)
    ON DELETE CASCADE

);