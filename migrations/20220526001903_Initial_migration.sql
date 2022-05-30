-- Add migration script here
CREATE TABLE urls (
    link VARCHAR(1024) UNIQUE NOT NULL,
    destination VARCHAR(1024) NOT NULL
);

CREATE TABLE accounts (
    username VARCHAR(64) UNIQUE NOT NULL,
    password VARCHAR(1024) NOT NULL
);