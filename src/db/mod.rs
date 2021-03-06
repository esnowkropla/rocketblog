use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};

use rusqlite;
use r2d2;

pub mod connection_manager;
use self::connection_manager::SqliteConnectionManager;
use users::NewUser;

type Pool = r2d2::Pool<SqliteConnectionManager>;

pub fn init_pool() -> Pool {
    let config = r2d2::Config::default();
    let manager = SqliteConnectionManager::new("db.sqlite");
    r2d2::Pool::new(config, manager).expect("db pool")
}

pub struct DbConn(pub r2d2::PooledConnection<SqliteConnectionManager>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, ()> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = rusqlite::Connection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
pub fn register_user(user: NewUser, conn: DbConn) -> Result<()> {
    let mut stmt = try!(conn.prepare_cached("INSERT INTO users (email, username, password_hash, role_id) VALUES (?)"));

    try!(stmt.execute(&[]));
}*/

pub fn mock_database()
{
    let create = "CREATE TABLE roles(
	id INTEGER PRIMARY KEY,
	name TEXT NOT NULL,
	permissions INTEGER
);

CREATE TABLE users(
	id INTEGER PRIMARY KEY,
	email TEXT NOT NULL,
	username TEXT NOT NULL,
	password_hash TEXT NOT NULL,
	role_id INTEGER NOT NULL,
	FOREIGN KEY(role_id) REFERENCES role(id)
);";
}


