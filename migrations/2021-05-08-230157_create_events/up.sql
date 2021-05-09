-- Your SQL goes here
CREATE TABLE events (
    id VARCHAR(36) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    location VARCHAR(100) NOT NULL,
    host VARCHAR(40) NOT NULL
)
