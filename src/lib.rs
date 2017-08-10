#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use rocket::response::{NamedFile, Failure, Redirect, status};
use rocket::http::{Status, Cookies, Cookie};
use rocket::request::{Request, Form};

use std::path::{Path, PathBuf};

use rocket_contrib::Template;

pub mod useragent;

#[derive(Serialize)]
struct Context<'a> {
    name: &'a str,
}

#[get("/<filename..>")]
fn static_files(filename: PathBuf) -> Result<NamedFile, Failure> {
    let path = Path::new("static/").join(filename);
    match NamedFile::open(&path) {
        Ok(file) => Ok(file),
        Err(e) => {
            println!("failed to find file {:?}, err: {}", path, e);
            return Err(Failure(Status::NotFound));
        }
    }
}

#[get("/about")]
fn about() -> Template {
    let context = Context { name: "" };
    Template::render("about", &context)
}

#[get("/contact")]
fn contact() -> Template {
    let context = Context { name: "" };
    Template::render("contact", &context)
}

#[derive(FromForm, Debug)]
struct NameForm {
    name: String,
}

#[post("/", data = "<input>")]
fn name_form(input: Form<NameForm>, mut cookies: Cookies) -> Redirect {
    cookies.add_private(Cookie::new("name", input.get().name.clone()));
    return Redirect::to("/");
}

#[get("/")]
fn index(mut cookies: Cookies) -> Template {
    match cookies.get_private("name") {
        Some(ref cookie) => Template::render("index", &Context { name: cookie.value() }),
        None => Template::render("index", &Context { name: "World!" }),
    }
}

#[error(404)]
fn not_found(_: &Request) -> status::NotFound<Template> {
    let context = Context { name: "" };
    status::NotFound(Template::render("404", &context))
}

#[error(500)]
fn server_error(_: &Request) -> status::Custom<Template> {
    let context = Context { name: "" };
    status::Custom(
        Status::InternalServerError,
        Template::render("404", &context),
    )
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![static_files, index, contact, about, name_form])
        .catch(errors![not_found, server_error])
        .attach(Template::fairing())
}
