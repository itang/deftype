use iron::prelude::*;
use iron::status;
use chrono::*;
// use persistent::Read;
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
    json_box(&Box::new(global::server_config().run_mode.to_str()))
}

pub fn users_list(_: &mut Request) -> IronResult<Response> {
    let conn = models::establish_connection();
    let users = models::find_users(&conn);
    json(&users)
}

pub fn users_create(req: &mut Request) -> IronResult<Response> {
    let parsed = req.get::<bodyparser::Struct<models::NewUser>>();
    let parsed = try!(parsed.map_err(|err| CustomBodyError { cause: err }));
    match parsed {
        Some(ref new_user) => json(&models::create_user(&models::establish_connection(), new_user)),
        None => json(&"".to_owned()),
    }
}

pub fn dev_mock_error(_: &mut Request) -> IronResult<Response> {
    Err(IronError::new(MockError, status::InternalServerError))
}
