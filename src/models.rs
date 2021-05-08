use rocket_contrib::databases::diesel;

#[database("scheduler_events")]
struct SchedulerDbConn(diesel::MysqlConnection);

#[derive(Queryable)]
pub struct Event {
    pub id: String,
    pub name: String,
    pub description: String,
    pub location: String,
    pub host: String
}

#[derive(Queryable, Associations)]
#[belongs_to(Event)]
pub struct Invite {
    pub id: String,
    pub event: String,
    pub invitee: String,
}

#[derive(Queryable, Associations)]
#[belongs_to(Event)]
pub struct Options {
    pub id: i32,
    pub description: String,
    pub count: i32,
}
