use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn create_user<'a>(conn: &PgConnection, new_user: &NewUser) -> User {
    use schema::users;

    diesel::insert(new_user)
        .into(users::table)
        .get_result(conn)
        .expect("Error saving new User")
}

pub fn find_users<'a>(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;
    let ret: Vec<User> = users.filter(valid.eq(true))
                              .limit(5)
                              .load::<User>(conn)
                              .expect("Error loading user");
    ret
}


#[derive(Queryable, RustcEncodable, RustcDecodable, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub login_name: String,
    pub password: String,
    pub valid: bool,
}

use super::schema::users;

#[insertable_into(users)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NewUser {
    pub login_name: String,
    pub password: String,
}


#[test]
fn test_establish_connection() {
    let _ = establish_connection();
}