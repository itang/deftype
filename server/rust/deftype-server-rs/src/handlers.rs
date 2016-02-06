extern crate rustc_serialize;
extern crate iron;
extern crate chrono;


use std::convert::From;

use rustc_serialize::Encodable;
use rustc_serialize::json::{self, EncoderError};
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use self::chrono::*;


pub fn welcome(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello from Rust!!")))
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
struct ServerTime {
    now: String,
}

pub fn server_time(_: &mut Request) -> IronResult<Response> {
    let dt = Local::now();
    let server_time = ServerTime { now: dt.format("%Y-%m-%d %H:%M:%S").to_string() };

    json(&server_time)
}

/// ///////////////////////////////////////////////////////////////////////////////////////////////
struct CustomEncoderError {
    cause: EncoderError,
}

impl From<CustomEncoderError> for IronError {
    fn from(err: CustomEncoderError) -> IronError {
        IronError::new(Box::new(err.cause),
                       (status::InternalServerError, "encode json error"))
    }
}

/// ///////////////////////////////////////////////////////////////////////////////////////////////
/// Private
fn json<T: Encodable>(value: &T) -> IronResult<Response> {
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    let s = try!(json::encode(value).map_err(|err| CustomEncoderError { cause: err }));

    Ok(Response::with((content_type, status::Ok, s)))
}
