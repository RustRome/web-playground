#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
mod entity;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}