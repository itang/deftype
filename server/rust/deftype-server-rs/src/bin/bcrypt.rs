#[macro_use]
extern crate deftype_server_rs;

use std::env;
use deftype_server_rs::util;


fn main() {
    let plain = env::args().nth(1).expect("输入要hash的串");

    let hashed = match util::bcrypt_hash(&plain) {
        Ok(h) => h,
        Err(_) => panic!("hash出错了."),
    };

    info!("{}", hashed);
}
