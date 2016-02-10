#![feature(const_fn, custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate hyper;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate bodyparser;

use std::{io, process};
use std::io::prelude::*;
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
mod models;
mod schema;


fn main() {
    let conf = global::server_config();

    let mut api_router = Router::new();
    api_router.get("/", handlers::welcome);
    api_router.get("/server/time", handlers::server_time);
    api_router.get("/server/mode", handlers::server_mode);

    api_router.get("/users", handlers::users_list);
    api_router.post("/users", handlers::users_create);

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
        process::exit(1);
    }
}
