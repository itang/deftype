use iron::prelude::*;
use iron::status;
use chrono::*;
use bodyparser;

use util::*;
use types::*;

use global;
use models;


pub fn welcome(_: &mut Request) -> IronResult<Response> {
    let msg = "Hello from Rust!";

    Ok(Response::with((status::Ok, msg)))
}

pub fn server_time(_: &mut Request) -> IronResult<Response> {
    let dt = Local::now();
    let server_time = ServerTime::new(dt.format("%Y-%m-%d %H:%M:%S").to_string());

    ResultDTO::ok(server_time).to_json_result()
}

pub fn server_mode(_: &mut Request) -> IronResult<Response> {
    json(&ResultDTO::ok(&global::server_config().run_mode.to_str()))
}

pub fn users_list(_: &mut Request) -> IronResult<Response> {
    let conn = try!(global::conn_pool().get().map_err(GetTimeoutWrapper));
    let users = models::find_users(&conn);
    json_result(ResultDTO::ok(users).code(200).message("获取用户成功!"))
}

pub fn users_create(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<bodyparser::Struct<models::NewUser>>();
    let parsed = try!(parsed.map_err(BodyErrorWrapper));
    match parsed {
        Some(ref new_user) => {
            let conn = try!(global::conn_pool().get().map_err(GetTimeoutWrapper));
            ResultDTO::ok(models::create_user(&conn, new_user)).to_json_result()
        }
        None => ResultDTO::err(&"".to_owned()).to_json_result(),
    }
}

pub fn dev_mock_error(_: &mut Request) -> IronResult<Response> {
    Err(IronError::new(MockError, status::InternalServerError))
}
