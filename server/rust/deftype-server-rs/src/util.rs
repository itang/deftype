use std::fmt;
use std::error::Error;
use iron::prelude::*;
use iron::{status, Handler};
use iron::mime::Mime;
use router::NoRoute;
use serde_json as json;
use serde_json::error::Error as JsonError;
use serde::ser;
use r2d2::GetTimeout;
use staticfile::Static;
use bodyparser::BodyError;

use types::ResultDTO;


pub struct JsonEncodeErrorWrapper(pub JsonError);

impl From<JsonEncodeErrorWrapper> for IronError {
    fn from(wrapper: JsonEncodeErrorWrapper) -> IronError {
        IronError::new(Box::new(wrapper.0),
                       (status::InternalServerError, "json encode error"))
    }
}

pub struct BodyErrorWrapper(pub BodyError);

impl From<BodyErrorWrapper> for IronError {
    fn from(wrapper: BodyErrorWrapper) -> IronError {
        IronError::new(Box::new(wrapper.0),
                       (status::InternalServerError, "body parse error"))
    }
}

pub struct GetTimeoutWrapper(pub GetTimeout);

impl From<GetTimeoutWrapper> for IronError {
    fn from(wrapper: GetTimeoutWrapper) -> IronError {
        IronError::new(Box::new(wrapper.0),
                       (status::InternalServerError, "get db connection timeout"))
    }
}


#[macro_export]
macro_rules! info {
    ($fmt: expr) => ({
        extern crate ansi_term;
        println!("{}: {}", ansi_term::Colour::Green.paint("INFO"), $fmt);
    });
    ($fmt: expr, $($arg: tt)*) => ({
        extern crate ansi_term;
        //let args = format!($fmt, $($arg)*);
        println!(concat!("{}: ", $fmt), ansi_term::Colour::Green.paint("INFO"), $($arg)*);
    });
}

#[macro_export]
macro_rules! warn {
    ($fmt: expr) => ({
        extern crate ansi_term;
        println!("{}: {}", ansi_term::Colour::Yellow.paint("WARN"), $fmt);
    });
    ($fmt: expr, $($arg: tt)*) => ({
        extern crate ansi_term;
        let args = format!($fmt, $($arg)*);
        println!("{}: {}", ansi_term::Colour::Yellow.paint("WARN"), args);
    });
}

#[macro_export]
macro_rules! error {
    ($fmt: expr) => ({
        extern crate ansi_term;
        println!("{}: {}", ansi_term::Colour::Red.paint("ERROR"), $fmt);
    });
    ($fmt: expr, $($arg: tt)*) => ({
        extern crate ansi_term;
        let args = format!($fmt, $($arg)*);
        println!("{}: {}", ansi_term::Colour::Red.paint("ERROR"), args);
    });
}

header! { (XMLHttpRequest, "X-Requested-With") => [String] }

header! { (XRuntime, "X-Runtime") => [String] }

#[allow(dead_code)]
pub fn is_ajax_request(req: &Request) -> bool {
    req.headers.has::<XMLHttpRequest>()
}

#[inline]
pub fn json<T>(value: &T) -> IronResult<Response>
    where T: ser::Serialize
{
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    let s = try!(json::to_string(value).map_err(JsonEncodeErrorWrapper));

    Ok(Response::with((content_type, status::Ok, s)))
}

#[inline]
pub fn json_result<T>(value: ResultDTO<T>) -> IronResult<Response>
    where T: ser::Serialize
{
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    let s = try!(json::to_string(&value).map_err(JsonEncodeErrorWrapper));

    Ok(Response::with((content_type, status::Ok, s)))
}

impl<T: ser::Serialize> ResultDTO<T> {
    pub fn to_json_result(&self) -> IronResult<Response> {
        json(self)
    }
}

pub struct MyStatic(pub Static);

impl Handler for MyStatic {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ret = self.0.handle(req);
        match ret {
            Ok(_) => ret,
            Err(e) => {
                warn!("{}", e);
                Err(IronError::new(NoRoute, status::NotFound))
            }
        }
    }
}


#[derive(Debug)]
pub struct MockError;

impl fmt::Display for MockError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("mock error.")
    }
}

impl Error for MockError {
    fn description(&self) -> &str {
        "Mock Error"
    }
}
