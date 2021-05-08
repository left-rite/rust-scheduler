-- Your SQL goes here
CREATE TABLE invitations {
    id GUID PRIMARY KEY,
    eventID GUID NOT NULL,
    invitee STRING NOT NULL,
    FOREIGN KEY(eventID) REFERENCES events(id)
}