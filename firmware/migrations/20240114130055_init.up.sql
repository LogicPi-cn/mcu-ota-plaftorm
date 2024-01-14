-- Add up migration script here

CREATE TABLE IF NOT EXISTS firmware_data (
    id SERIAL PRIMARY KEY,
    info TEXT NOT NULL,
    data BYTEA NOT NULL
);
