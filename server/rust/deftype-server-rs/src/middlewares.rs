use iron::{BeforeMiddleware, AfterMiddleware, typemap, status};
use iron::prelude::*;
use router::NoRoute;
use time::precise_time_ns;

use hyper::header;
use hyper::header::{Authorization, Bearer};
use crypto::sha2::Sha256;
use jwt::{Header, Registered, Token};

use util::*;
use global::*;


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
        warn!("{}", err);
        if let Some(_) = err.error.downcast::<NoRoute>() {
            // TODO: custom 400 page.
            Ok(Response::with((status::NotFound, "Not Found")))
        } else {
            // Err(err)
            // TODO: custom 500 page.
            Ok(Response::with((status::InternalServerError, "500")))
        }
    }
}


pub struct JwtFilter;

impl BeforeMiddleware for JwtFilter {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        let uri = &req.url.path.join("/");
        if uri.starts_with("api/") && !uri.starts_with("api/users/login") {
            // Get the full Authorization header from the incoming request headers
            let auth_header = match req.headers.get::<Authorization<Bearer>>() {
                Some(header) => header,
                None => panic!("No authorization header found"),
            };

            // Format the header to only take the value
            let jwt = header::HeaderFormatter(auth_header).to_string();

            // We don't need the Bearer part,
            // so get whatever is after an index of 7
            let jwt_slice = &jwt[7..];

            // Parse the token
            let token = Token::<Header, Registered>::parse(jwt_slice).unwrap();

            // Get the secret key as bytes
            let secret = server_config().auth_secret.as_bytes();
            // Verify the token
            if !token.verify(secret, Sha256::new()) {
                return Err(IronError::new(StringError("授权不通过!".to_string()),
                                          (status::Forbidden, "授权不通过!")));
            }
        }

        Ok(())
    }
}
