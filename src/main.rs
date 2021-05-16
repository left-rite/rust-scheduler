#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] 
extern crate diesel;

mod models;
mod schema;
mod dtos;

use rocket_contrib::templates::Template;
use rocket::request::Form;
use dtos::ProposedEvent;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/create")]
fn create_event_page() -> Template {
    Template::render("create_event", {})
}

#[post("/create", data = "<proposed_event>")]
fn create_event(proposed_event: Form<ProposedEvent>) -> String {
    format!("New event: {} \n {}", proposed_event.name, proposed_event.description)
}

fn main() {
    rocket::ignite()
       .attach(models::SchedulerDbConn::fairing())
       .mount("/", routes![index])
       .mount("/events", routes![create_event_page, create_event])
    //    .mount("/events/show", routes![index])
    //    .mount("/events/edit", routes![index])
    //    .mount("/events/signup", routes![index])
       .attach(Template::fairing())
       .launch();
}