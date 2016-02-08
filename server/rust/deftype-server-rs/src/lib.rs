#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(diesel_codegen, dotenv_macros))]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate rustc_serialize;

#[cfg(feature = "nightly")]
include!("lib.in.rs");

#[cfg(feature = "with-syntex")]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

use self::models::{User, NewUser};

pub fn create_user<'a>(conn: &PgConnection, login_name: &'a str, password: &'a str) -> User {
    use schema::users;

    let new_user = NewUser {
        login_name: login_name,
        password: password,
    };

    diesel::insert(&new_user).into(users::table)
        .get_result(conn)
        .expect("Error saving new User")
}

pub fn find_users<'a>(conn: &PgConnection) -> Vec<User> {
    use self::schema::users::dsl::*;
    let ret: Vec<User> = users.filter(valid.eq(true))
        .limit(5)
        .load::<User>(conn)
        .expect("Error loading user");
    ret
}

#[test]
fn test_establish_connection() {
    let _ = establish_connection();
}
