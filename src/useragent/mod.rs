use rocket::request::{self, FromRequest};
use rocket::{Request, Outcome};

#[derive(Debug)]
pub struct UserAgent<'a>(&'a str);

impl<'a, 'r> FromRequest<'a, 'r> for UserAgent<'a> {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<UserAgent<'a>, ()> {
        match request.headers().get_one("User-Agent") {
            Some(text) => Outcome::Success(UserAgent(text)),
            None => Outcome::Forward(()),
        }
    }
}
