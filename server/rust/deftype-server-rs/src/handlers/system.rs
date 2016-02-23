use iron::prelude::*;
use chrono::*;

use types::*;
use global;


pub fn server_time(_: &mut Request) -> IronResult<Response> {
    let dt = Local::now();
    let server_time = ServerTime::new(dt.format("%Y-%m-%d %H:%M:%S").to_string());

    ResultDTO::ok(server_time).json_result()
}

pub fn server_mode(_: &mut Request) -> IronResult<Response> {
    ResultDTO::ok(&global::server_config().run_mode.to_str()).json_result()
}
