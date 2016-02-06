// extern crate hyper;
extern crate iron;
extern crate router;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
// use iron::mime::Mime;
use router::NoRoute;
use self::time::precise_time_ns; // why need self::??

/// ////////////////////////////////////////////////////////////
// from hyper.
header! {
    (XRuntime, "X-Runtime") => [String]
}

pub struct Runtime;

impl typemap::Key for Runtime {
    type Value = u64;
}

impl BeforeMiddleware for Runtime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        req.extensions.insert::<Runtime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for Runtime {
    fn after(&self, req: &mut Request, mut res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<Runtime>().unwrap();

        let xrstr = format!("{} ms", (delta as f64) / 1000000.0);
        // println!("Request took: {}", xrstr);
        res.headers.set(XRuntime(xrstr));

        Ok(res)
    }
}

/// ////////////////////////////////////////////////////////////////////////
pub struct ErrorsHandler;

impl AfterMiddleware for ErrorsHandler {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "NoFound")))
        } else {
            // TODO: more
            Err(err)
        }
    }
}
