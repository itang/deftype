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
mod types;


fn main() {
    let config = config::ServerConfig::load_from_env();

    let mut api_router = Router::new();
    api_router.get("/", handlers::welcome);
    api_router.get("/server/time", handlers::server_time);

    let mut dev_router = Router::new();
    dev_router.get("/mock/error", handlers::dev_mock_error);

    let mut mount = Mount::new();
    mount.mount("/api", api_router);
    mount.mount("/_dev", dev_router);
    mount.mount("/",
                util::MyStatic(Static::new(Path::new(&config.root_dir))));

    let mut chain = Chain::new(mount);
    chain.link_before(middlewares::Runtime);
    chain.link_after(middlewares::ErrorsHandler);
    chain.link_after(middlewares::Runtime);

    info!("Serve on {}:{} ...", config.host, config.port);

    if let Err(e) = Iron::new(chain).http(config.to_addr()) {
        let _ = writeln!(io::stderr(), "Error: {}.", e);
    }
}
