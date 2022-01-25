-- Add migration script here
CREATE TABLE IF NOT EXISTS links
(
    link text NOT NULL,
    destination text NOT NULL
)
