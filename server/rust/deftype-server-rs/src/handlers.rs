use iron::prelude::*;
use iron::status;
use chrono::*;

use util::*;
use types::*;

pub fn welcome(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello from Rust!!")))
}

pub fn server_time(_: &mut Request) -> IronResult<Response> {
    let dt = Local::now();
    let server_time = ServerTime::new(dt.format("%Y-%m-%d %H:%M:%S").to_string());

    json(&server_time)
}


pub fn dev_mock_error(_: &mut Request) -> IronResult<Response> {
    Err(IronError::new(MockError, status::InternalServerError))
}
