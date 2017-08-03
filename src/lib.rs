#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;

use rocket::http::{Cookie, Cookies};
use rocket::response::Redirect;
use rocket::request::Form;

use rocket_contrib::Template;

pub mod useragent;

#[derive(Serialize)]
struct UserContext<'a> {
    name: &'a str,
}

#[derive(Serialize)]
struct Blank;

#[derive(FromForm)]
struct Name {
    pub name: String,
}

#[post("/", data = "<user_form>")]
fn name(user_form: Form<Name>, mut cookies: Cookies) -> Redirect {
    cookies.add_private(Cookie::new("name", user_form.get().name.clone()));
    Redirect::to("/")
}

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove_private(Cookie::named("name"));
    Redirect::to("/")
}

#[get("/")]
fn index(mut cookies: Cookies) -> Template {
    let blank = Blank;
    match cookies.get_private("name") {
        Some(cookie) => {
            let context = UserContext { name: cookie.value() };
            return Template::render("user", &context);
        }
        None => return Template::render("index", &blank),
    };
}

#[get("/user/<name>")]
fn user(name: String) -> Template {
    let context = UserContext { name: &name };
    Template::render("user", context)
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, user, name, logout])
        .attach(Template::fairing())
}
