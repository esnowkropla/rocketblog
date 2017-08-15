#![feature(custom_derive)]
#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

extern crate rusqlite;
extern crate r2d2;
extern crate chrono;
extern crate bcrypt;

use rocket::response::{NamedFile, Failure, Redirect, status};
use rocket::http::{Status, Cookies, Cookie};
use rocket::request::{Request, Form};

use serde_json::to_string;

use std::path::{Path, PathBuf};

use bcrypt::{DEFAULT_COST, hash, verify};
use rocket_contrib::{Template, Json};

pub mod useragent;
pub mod db;
pub mod users;
pub mod posts;

use db::DbConn;
use users::User;

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

#[derive(Serialize, Deserialize)]
struct Post {
    id: i32,
    body: String,
    date: String,
}

#[get("/posts")]
fn posts(conn: DbConn) -> Json<Vec<Post>> {
    let mut stmt = conn.prepare("SELECT id, body, date FROM posts").unwrap();
    let rows = stmt.query_map(&[], |row| {
        Post {
            id: row.get(0),
            body: row.get(1),
            date: row.get(2),
        }
    }).unwrap();

    let mut posts = Vec::new();
    for row in rows {
        if let Ok(post) = row {
            posts.push(post)
        }
    }
    return Json(posts);
}

#[derive(FromForm, Debug)]
struct Login {
    email: String,
    password: String,
}

#[post("/login", data = "<input>")]
fn post_login(input: Form<Login>, conn: DbConn, mut cookies: Cookies) -> Redirect {
    let mut stmt = conn.prepare("SELECT * FROM users WHERE email=?").unwrap();
    let user = match stmt.query_row(&[&input.get().email], |x| User::from_row(x)) {
        Ok(user) => user,
        Err(_) => return Redirect::to("/"),
    };

    match hash(&input.get().password, DEFAULT_COST) {
        Err(_) => return Redirect::to("/"),
        _ => (),
    };

    cookies.add_private(Cookie::new("auth", to_string(&user).unwrap()));

    Redirect::to("/")
}

#[get("/login")]
fn get_login() -> Template {
    Template::render("login", &Context { name: "" })
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
        .mount(
            "/",
            routes![
                static_files,
                index,
                contact,
                about,
                name_form,
                posts,
                get_login,
                post_login,
            ],
        )
        .catch(errors![not_found, server_error])
        .manage(db::init_pool())
        .attach(Template::fairing())
}
