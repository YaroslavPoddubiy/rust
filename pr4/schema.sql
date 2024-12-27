-- schema.sql
-- DROP TABLE users;
-- DROP TABLE messages;
-- DROP TABLE chats;
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT UNIQUE NOT NULL,
    password TEXT NOT NULL
);

-- CREATE TABLE chats (
--     id INTEGER PRIMARY KEY AUTOINCREMENT,
--     username1 TEXT NOT NULL,
--     username2 TEXT NOT NULL,
--     FOREIGN KEY (username1) REFERENCES users(id),
--     FOREIGN KEY (username2) REFERENCES users(id)
-- );

CREATE TABLE messages (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL,
    text TEXT NOT NULL,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (username) REFERENCES users(username) ON DELETE CASCADE
);
