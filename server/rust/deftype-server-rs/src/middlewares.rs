use iron::prelude::*;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use iron::status;
use router::NoRoute;
use time::precise_time_ns;

use util::*;


pub struct Runtime;

impl typemap::Key for Runtime {
    type Value = u64;
}

impl BeforeMiddleware for Runtime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        // info!("req {} is ajax request:{}", req.url, is_ajax_request(req));
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
            // TODO: custom 505 page.
            Ok(Response::with((status::NotFound, "Not Found")))
        } else {
            //Err(err)
            //TODO: custom 505 page.
            Ok(Response::with((status::InternalServerError, "500")))
        }
    }
}
