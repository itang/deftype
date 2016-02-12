use iron::prelude::*;
use iron::status;
use chrono::*;
use bodyparser;

use util::*;
use types::*;

use global;
use models::users;

pub fn welcome(_: &mut Request) -> IronResult<Response> {
    let msg = "Hello from Rust!";

    Ok(Response::with((status::Ok, msg)))
}

pub fn server_time(_: &mut Request) -> IronResult<Response> {
    let dt = Local::now();
    let server_time = ServerTime::new(dt.format("%Y-%m-%d %H:%M:%S").to_string());

    ResultDTO::ok(server_time).json_result()
}

pub fn server_mode(_: &mut Request) -> IronResult<Response> {
    ResultDTO::ok(&global::server_config().run_mode.to_str()).json_result()
}

pub fn users_list(_: &mut Request) -> IronResult<Response> {
    let conn = try!(global::conn_pool().get().map_err(GetTimeoutWrapper));
    let users = users::find_users(&conn);

    ResultDTO::ok(users).code(200).message("获取用户成功!").json_result()
}

pub fn users_create(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<bodyparser::Struct<users::NewUser>>();
    let parsed = try!(parsed.map_err(BodyErrorWrapper));
    match parsed {
        Some(ref new_user) => {
            let conn = try!(global::conn_pool().get().map_err(GetTimeoutWrapper));
            ResultDTO::ok(users::create_user(&conn, new_user)).json_result()
        }
        None => ResultDTO::err("").json_result(),
    }
}

pub fn users_login(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<bodyparser::Struct<users::LoginForm>>();
    let parsed = try!(parsed.map_err(BodyErrorWrapper));
    match parsed {
        Some(ref login_form) => {
            let conn = try!(global::conn_pool().get().map_err(GetTimeoutWrapper));
            match users::login(&conn, login_form) {
                Some(user) => ResultDTO::ok(&user).json_result(),
                None => ResultDTO::err("用户不存在或者密码输入有误").json_result(),
            }
        }
        None => ResultDTO::err("").json_result(),
    }
}

pub fn dev_mock_error(_: &mut Request) -> IronResult<Response> {
    Err(IronError::new(MockError, status::InternalServerError))
}
