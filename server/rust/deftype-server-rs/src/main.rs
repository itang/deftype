#[macro_use]
extern crate hyper;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate rustc_serialize;
extern crate time;
extern crate chrono;
#[macro_use]
extern crate lazy_static;

use std::io::prelude::*;
use std::io;
use std::path::Path;

use iron::prelude::*;
use router::Router;
use mount::Mount;
use staticfile::Static;


mod global;
mod config;
#[macro_use]mod util;
mod middlewares;
mod handlers;
mod types;


fn main() {
    let conf: &config::ServerConfig = &(*global::SERVER_CONFIG);

    let mut api_router = Router::new();
    api_router.get("/", handlers::welcome);
    api_router.get("/server/time", handlers::server_time);
    api_router.get("/server/mode", handlers::server_mode);


    let mut mount = Mount::new();
    mount.mount("/api", api_router);

    if conf.run_mode.is_dev() {
        let mut dev_router = Router::new();
        dev_router.get("/mock/error", handlers::dev_mock_error);
        mount.mount("/_dev", dev_router);
    }

    mount.mount("/", util::MyStatic(Static::new(Path::new(&conf.root_dir))));

    let mut chain = Chain::new(mount);
    chain.link_before(middlewares::Runtime);
    chain.link_after(middlewares::ErrorsHandler);
    chain.link_after(middlewares::Runtime);

    info!("[{}] Serve on {}:{} ...",
          conf.run_mode,
          conf.host,
          conf.port);

    if let Err(e) = Iron::new(chain).http(conf.to_addr()) {
        let _ = writeln!(io::stderr(), "Error: {}.", e);
    }
}
