CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(32) NOT NULL UNIQUE,
    name VARCHAR(32) NOT NULL,
    last_name VARCHAR(100),
    password_hash TEXT NOT NULL
);