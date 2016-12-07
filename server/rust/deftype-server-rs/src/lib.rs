#![feature(proc_macro)]

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
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate time;
extern crate chrono;
#[macro_use]
extern crate lazy_static;
extern crate bodyparser;
extern crate bcrypt;
extern crate jwt;
extern crate crypto;


pub mod global;
pub mod config;
#[macro_use]
pub mod util;
pub mod middlewares;
pub mod handlers;
pub mod types;
pub mod models;

mod schema;
