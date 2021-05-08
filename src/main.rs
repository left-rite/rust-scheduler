#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use models;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite()
       .attach(SchedulerDbConn::fairing())
       .mount("/", routes![index])
    //    .mount("/events/create", routes![index])
    //    .mount("/events/show", routes![index])
    //    .mount("/events/edit", routes![index])
    //    .mount("/events/signup", routes![index])
       .launch();
}