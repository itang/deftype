extern crate bcrypt;

use std::env;

#[allow(unused_imports)]
use bcrypt::{DEFAULT_COST, hash, verify};


fn main() {
    let plain = env::args().nth(1).expect("输入要hash的串");

    let hashed = match hash(&plain, DEFAULT_COST) {
        Ok(h) => h,
        Err(_) => panic!("hash出错了."),
    };

    println!("{}", hashed);
}
