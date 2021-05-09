-- Your SQL goes here
CREATE TABLE options (
    id INT PRIMARY KEY,
    event_id VARCHAR(36) NOT NULL,
    description TEXT NOT NULL,
    count INT DEFAULT 0,
    FOREIGN KEY(event_id) REFERENCES events(id)
)