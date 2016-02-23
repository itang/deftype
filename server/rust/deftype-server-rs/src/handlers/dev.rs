use iron::prelude::*;
use iron::status;

use util::*;


pub fn mock_error(_: &mut Request) -> IronResult<Response> {
    Err(IronError::new(MockError, status::InternalServerError))
}
