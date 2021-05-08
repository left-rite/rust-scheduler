-- Your SQL goes here
CREATE TABLE events {
    id GUID PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    location VARCHAR NOT NULL,
    host VARCHAR NOT NULL
}