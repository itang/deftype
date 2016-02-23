use iron::prelude::*;
use iron::status;


pub mod dev;
pub mod system;
pub mod users;


pub fn welcome(_: &mut Request) -> IronResult<Response> {
    let msg = "Hello from Rust!";

    Ok(Response::with((status::Ok, msg)))
}
