#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] 
extern crate diesel;

mod models;
mod schema;

use rocket_contrib::templates::Template;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/create")]
fn create() -> Template {
    Template::render("create_event", {})
}

fn main() {
    rocket::ignite()
       .attach(models::SchedulerDbConn::fairing())
       .mount("/", routes![index])
       .mount("/events", routes![create])
    //    .mount("/events/show", routes![index])
    //    .mount("/events/edit", routes![index])
    //    .mount("/events/signup", routes![index])
       .attach(Template::fairing())
       .launch();
}