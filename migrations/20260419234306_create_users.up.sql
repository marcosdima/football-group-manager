CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(32) NOT NULL,
    last_name VARCHAR(100),
    password_hash TEXT NOT NULL
);