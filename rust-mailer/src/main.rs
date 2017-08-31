#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;


#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

mod entity;

#[get("/")]
fn hello() -> &'static str {    
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}