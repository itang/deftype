extern crate iron;

use iron::prelude::*;
use iron::status;

pub fn welcome(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello from Rust!!")))
}
