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

    json(&server_time)
}

pub fn server_mode(_: &mut Request) -> IronResult<Response> {
    json(&global::server_config().run_mode.to_str())
}

pub fn users_list(_: &mut Request) -> IronResult<Response> {
    let conn = try!(global::conn_pool().get().map_err(GetTimeoutWrapper));
    let users = models::find_users(&conn);
    json(&users)
}

pub fn users_create(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<bodyparser::Struct<models::NewUser>>();
    let parsed = try!(parsed.map_err(BodyErrorWrapper));
    match parsed {
        Some(ref new_user) => {
            let conn = try!(global::conn_pool().get().map_err(GetTimeoutWrapper));
            json(&models::create_user(&conn, new_user))
        }
        None => json(&"".to_owned()),
    }
}

pub fn dev_mock_error(_: &mut Request) -> IronResult<Response> {
    Err(IronError::new(MockError, status::InternalServerError))
}
