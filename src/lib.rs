#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde;

use rocket::http::{Cookie, Cookies};
use rocket::response::{Redirect, Flash};
use rocket::request::{self, Form, FlashMessage, FromRequest, Request};
use rocket::Outcome;

use rocket_contrib::Template;

pub mod useragent;
pub mod flasher;

struct UserContext<'a> {
    pub name: Option<&'a str>,
    flash: Option<flasher::Flasher>,
}

struct Admin;

impl<'a, 'r> FromRequest<'a, 'r> for Admin {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Admin, ()> {
        match request.cookies().get_private("name") {
            Some(ref cookie) if cookie.value() == "Elliot" => Outcome::Success(Admin),
            _ => Outcome::Forward(()),
        }
    }
}

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
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("name"));
    Flash::success(Redirect::to("/"), "Successfully logged out")
}

#[get("/admin", rank = 1)]
fn admin_panel(admin: Admin, flash: Option<FlashMessage>) -> Template {
    let flasher = match flash {
        Some(msg) => Some(flasher::Flasher(msg)),
        None => None,
    };
    let context = UserContext {
        name: None,
        flash: flasher,
    };
    Template::render("admin", &context)
}

#[get("/admin", rank = 2)]
fn admin_panel_user() -> Flash<Redirect> {
    Flash::error(
        Redirect::to("/"),
        "Sorry, you're not allowed on the admin page",
    )
}

#[get("/")]
fn index(flash: Option<FlashMessage>, mut cookies: Cookies) -> Template {
    let flasher = match flash {
        Some(msg) => Some(flasher::Flasher(msg)),
        None => None,
    };
    match cookies.get_private("name") {
        Some(cookie) => {
            return Template::render(
                "user",
                &UserContext {
                    name: Some(cookie.value()),
                    flash: flasher,
                },
            );
        }
        None => {
            return Template::render(
                "index",
                &UserContext {
                    name: None,
                    flash: flasher,
                },
            )
        }
    };
}

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![index, name, logout, admin_panel, admin_panel_user],
        )
        .attach(Template::fairing())
}
