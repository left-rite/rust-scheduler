-- Your SQL goes here
CREATE TABLE options {
    id INT PRIMARY KEY,
    event GUID NOT NULL,
    description TEXT NOT NULL,
    count INT DEFAULT 0,
    FOREIGN KEY(event) REFERENCES events(id)
}