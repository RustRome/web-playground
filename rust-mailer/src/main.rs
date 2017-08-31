#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;

extern crate dotenv;



#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod entity;
mod db;

#[get("/")]
fn hello() -> &'static str {    
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}