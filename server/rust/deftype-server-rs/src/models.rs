#[derive(Queryable, RustcEncodable, RustcDecodable, Debug)]
pub struct User {
    pub id: i32,
    pub login_name: String,
    pub password: String,
    pub valid: bool,
}

use super::schema::users;

#[insertable_into(users)]
pub struct NewUser<'a> {
    pub login_name: &'a str,
    pub password: &'a str,
}
