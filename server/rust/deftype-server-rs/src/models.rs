use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use util::*;


pub fn create_user(conn: &PgConnection, new_user: &NewUser) -> User {
    use schema::users;

    let new_user = &{
        // TODO: 错误友好处理
        let new_user = new_user.cleaning().validate().expect("输入验证有误");
        let hashed_pwd = bcrypt_hash(&new_user.password).expect("hashed password error.");

        NewUser { password: hashed_pwd, ..new_user }
    };

    diesel::insert(new_user)
        .into(users::table)
        .get_result(conn)
        .expect("Error saving new User")
}

pub fn find_users(conn: &PgConnection) -> Vec<User> {
    use schema::users::dsl::*;

    users.filter(valid.eq(true))
         .limit(5)
         .load::<User>(conn)
         .expect("Error loading user")
}


#[derive(Queryable, Debug, Serialize)]
pub struct User {
    pub id: i32,
    pub login_name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub valid: bool,
}

use super::schema::users;

#[insertable_into(users)]
#[derive(Debug, Deserialize, Clone)]
pub struct NewUser {
    pub login_name: String,
    pub password: String,
}

type ValidResult<T> = Result<T, String>;

trait Valid<T> {
    fn validate(self) -> ValidResult<T>;
}

trait  Cleaning{
    fn cleaning(&self) -> Self;
}

impl Cleaning for NewUser {
    fn cleaning(&self) -> Self {
        let mut clone = self.clone();
        clone.login_name = clone.login_name.trim().to_owned();
        clone.password = clone.password.trim().to_owned();
        clone
    }
}

impl Valid<NewUser> for NewUser {
    fn validate(self) -> ValidResult<NewUser> {
        if self.login_name.len() < 2 {
            return Err("login_name min-length: 2".to_owned());
        }
        if self.password.len() < 6 {
            return Err("password min-length: 6".to_owned());
        }

        Ok(self)
    }
}
