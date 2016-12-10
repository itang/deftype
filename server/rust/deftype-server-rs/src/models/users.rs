use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;

use std::default::Default;
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};

use util::*;
use types::*;
use global::*;


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

pub fn login(conn: &PgConnection, login_form: &LoginForm) -> Option<LoginResponse> {
    use schema::users::dsl::*;
    let ret = users.filter(login_name.eq(&login_form.login_name))
        .limit(1)
        .load::<User>(conn)
        .expect("Error loading user");
    match ret.first() {
        Some(user) => {
            if bcrypt_verify(&login_form.password, &user.password) {
                let header: Header = Default::default();
                // For the example, we just have one claim
                // You would also want iss, exp, iat etc
                let claims =
                    Registered { sub: Some(user.login_name.clone()), ..Default::default() };
                let token = Token::new(header, claims);
                // Sign the token
                let jwt = token.signed(server_config().auth_secret.as_bytes(), Sha256::new())
                    .unwrap();
                let token_str = format!("{}", jwt);

                Some(LoginResponse::new(user.clone(), token_str))
            } else {
                None
            }
        }
        None => None,
    }
}


#[derive(Debug, Clone, Serialize, Queryable)]
pub struct User {
    pub id: i32,
    pub login_name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub valid: bool,
}

use schema::users;

#[derive(Insertable)]
#[table_name="users"]
#[derive(Debug, Clone, Deserialize)]
pub struct NewUser {
    pub login_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginForm {
    pub login_name: String,
    pub password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

impl LoginResponse {
    pub fn new(user: User, token: String) -> Self {
        LoginResponse {
            user: user,
            token: token,
        }
    }
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
    fn validate(&self) -> ValidResult<NewUser> {

        if self.login_name.len() < 2 {
            return Err("login_name min-length: 2".to_owned());
        }
        if self.password.len() < 6 {
            return Err("password min-length: 6".to_owned());
        }

        Ok(self.clone())
    }
}
