extern crate dotenv;
extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;
#[macro_use]
extern crate deftype_server_rs;


use std::{io, process};
use std::io::prelude::*;
use std::path::Path;
use iron::prelude::*;
use router::Router;
use mount::Mount;
use staticfile::Static;
use dotenv::dotenv;

use deftype_server_rs::{global, util, handlers, middlewares};


fn main() {
    dotenv().ok();

    let conf = global::server_config();

    let mut api_router = Router::new();
    api_router.get("/", handlers::welcome, "index");
    api_router.get("/server/time", handlers::system::server_time, "server_time");
    api_router.get("/server/mode", handlers::system::server_mode, "server_mode");

    api_router.get("/users", handlers::users::list, "get_users");
    api_router.post("/users", handlers::users::create, "create_users");
    api_router.post("/users/login", handlers::users::login, "login");

    let mut mount = Mount::new();
    mount.mount("/api", api_router);

    if conf.run_mode.is_dev() {
        let mut dev_router = Router::new();
        dev_router.get("/mock/error", handlers::dev::mock_error, "error");
        mount.mount("/_dev", dev_router);
    }

    mount.mount("/", util::MyStatic(Static::new(Path::new(&conf.root_dir))));

    let mut chain = Chain::new(mount);
    chain.link_before(middlewares::Runtime);
    chain.link_before(middlewares::JwtFilter);
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
