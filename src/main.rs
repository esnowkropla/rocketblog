#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocketblog;

use rocket::http::{RawStr, ContentType};
use rocket::response::Content;

use rocketblog::useragent::UserAgent;

#[get("/")]
fn index(user_agent: UserAgent) -> Content<String> {
    Content(
        ContentType::HTML,
        format!(
            "<h1>Hello, world!</h1>\nYour user agent is {:?}",
            user_agent
        ),
    )
}

#[get("/<name>")]
fn user(name: &RawStr) -> Content<String> {
    Content(
        ContentType::HTML,
        format!("<h1>Hello, {}!</h1>", name.as_str()),
    )
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![index, user])
}

fn main() {
    rocket().launch();
}
