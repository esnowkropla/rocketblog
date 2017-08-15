use rusqlite::Row;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    id: i32,
    email: String,
    username: String,
    password_hash: String,
    role_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub username: &'a str,
    pub password: &'a str,
    pub role: &'a str,
}

macro_rules! get_row {
    ($u:ident.$f:ident, $r:ident, $i:expr) => ($u.$f = match $r.get_checked($i) {
        Ok(val) => val,
        Err(_) => return None,
    };);
}

impl User {
    fn new() -> User {
        User {
            id: 0,
            email: String::from(""),
            username: String::from(""),
            password_hash: String::from(""),
            role_id: 0,
        }
    }

    pub fn from_row(row: &Row) -> Option<User> {
        let mut user = User::new();
        get_row!(user.id, row, 0);
        get_row!(user.email, row, 1);
        get_row!(user.username, row, 2);
        get_row!(user.password_hash, row, 3);
        get_row!(user.role_id, row, 4);
        Some(user)
    }
}
