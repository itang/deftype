use std::fmt;
use std::error::Error;
use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use iron::Handler;
use router::NoRoute;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self, EncoderError};
use staticfile::Static;


pub struct CustomEncoderError {
    cause: EncoderError,
}

impl From<CustomEncoderError> for IronError {
    fn from(err: CustomEncoderError) -> IronError {
        IronError::new(Box::new(err.cause),
                       (status::InternalServerError, "encode json error"))
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
        println!("{}: {}", ansi_term::Colour::Red.paint("WARN"), $fmt);
    });
    ($fmt: expr, $($arg: tt)*) => ({
        extern crate ansi_term;
        let args = format!($fmt, $($arg)*);
        println!("{}: {}", ansi_term::Colour::Red.paint("WARN"), args);
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

pub fn json<T: Encodable>(value: &T) -> IronResult<Response> {
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    let s = try!(json::encode(value).map_err(|err| CustomEncoderError { cause: err }));

    Ok(Response::with((content_type, status::Ok, s)))
}

pub fn json_box<T: ?Sized + Encodable>(value: &Box<T>) -> IronResult<Response> {
    let content_type = "application/json; charset=utf-8".parse::<Mime>().unwrap();
    let s = try!(json::encode(value).map_err(|err| CustomEncoderError { cause: err }));

    Ok(Response::with((content_type, status::Ok, s)))
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
