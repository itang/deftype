extern crate iron;
extern crate router;

use std::io::prelude::*;
use std::io;

use iron::prelude::*;
use router::Router;

mod config;
mod middlewares;
mod handlers;
#[macro_use]mod util;

fn main() {
    let mut router = Router::new();
    router.get("/", handlers::welcome);

    let mut chain = Chain::new(router);
    chain.link_before(middlewares::Runtime);
    chain.link_after(middlewares::ErrorsHandler);
    chain.link_after(middlewares::Runtime);

    let sc = config::ServerConfig::load_from_env();
    info!("Serve on {}:{} ...", sc.host, sc.port);
    if let Err(e) = Iron::new(chain).http(sc.to_addr()) {
        let _ = writeln!(io::stderr(), "Error: {}.", e);
    }
}
