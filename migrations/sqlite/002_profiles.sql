CREATE TABLE profiles (

    id INTEGER PRIMARY KEY AUTOINCREMENT,

    user_id INTEGER NOT NULL UNIQUE,

    username TEXT NOT NULL UNIQUE,

    first_name TEXT,

    last_name TEXT,

    avatar_path TEXT,


    FOREIGN KEY(user_id)
    REFERENCES users(id)
    ON DELETE CASCADE

);