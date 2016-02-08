extern crate deftype_server_rs;
use iron::prelude::*;
use iron::status;
use chrono::*;

use util::*;
use types::*;

use global;

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
    json_box(&Box::new(global::server_config().run_mode.to_str()))
}

pub fn users_list(_:&mut Request) -> IronResult<Response> {
    let conn = deftype_server_rs::establish_connection();
    let users = deftype_server_rs::find_users(&conn);
    json(&users)
}

pub fn dev_mock_error(_: &mut Request) -> IronResult<Response> {
    Err(IronError::new(MockError, status::InternalServerError))
}
