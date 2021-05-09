-- Your SQL goes here
CREATE TABLE invitations (
    id VARCHAR(36) PRIMARY KEY,
    event_id VARCHAR(36) NOT NULL,
    invitee VARCHAR(15) NOT NULL,
    FOREIGN KEY(event_id) REFERENCES events(id)
)