extern crate iron;
extern crate router;
extern crate time;

use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use router::NoRoute;
use self::time::precise_time_ns; // why need self::??

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
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<Runtime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);

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
