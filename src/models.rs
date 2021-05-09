use rocket_contrib::database;
use rocket_contrib::databases::diesel;

#[database("scheduler")]
pub struct SchedulerDbConn(diesel::MysqlConnection);

use crate::schema::{events, invitations, options};

#[derive(Identifiable, Queryable)]
#[table_name = "events"]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: String,
    pub location: String,
    pub host: String
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Event)]
#[table_name = "invitations"]
pub struct Invitation {
    pub id: String,
    pub event_id: String,
    pub invitee: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Event)]
#[table_name = "options"]
pub struct EventOption {
    pub id: i32,
    pub description: String,
    pub count: i32,
    pub event_id: String
}
