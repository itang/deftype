#[macro_use]
extern crate hyper;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
extern crate time;
extern crate chrono;


use std::io::prelude::*;
use std::io;
use std::path::Path;

use iron::prelude::*;
use router::Router;
use mount::Mount;
use staticfile::Static;

mod config;
#[macro_use]mod util;
mod middlewares;
mod handlers;


fn main() {
    let sc = config::ServerConfig::load_from_env();

    let mut api_router = Router::new();
    api_router.get("/", handlers::welcome);
    api_router.get("/server/time", handlers::server_time);

    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new(&sc.root_dir)));
    mount.mount("/api", api_router);

    let mut chain = Chain::new(mount);
    chain.link_before(middlewares::Runtime);
    chain.link_after(middlewares::ErrorsHandler);
    chain.link_after(middlewares::Runtime);

    info!("Serve on {}:{} ...", sc.host, sc.port);

    if let Err(e) = Iron::new(chain).http(sc.to_addr()) {
        let _ = writeln!(io::stderr(), "Error: {}.", e);
    }
}
