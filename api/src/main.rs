#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!(
        "Hello, {} year old dude going by the name of {}!",
        age, name
    )
}

#[derive(Debug, Serialize)]
struct Foo {
    name: String,
    message: String,
}

#[get("/yo/<name>")]
fn get_yo(name: String) -> Json<Foo> {
    let foo = Foo {
        name,
        message: String::from("Yo is great!"),
    };
    Json(foo)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, get_yo])
}
