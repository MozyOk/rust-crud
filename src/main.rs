#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} years old name {}!", age, name)
}


fn main() {
    rocket::ignite()
        .mount("/hello", routes![hello])
        .launch();
}