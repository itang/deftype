use std::env;
use r2d2;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use diesel::pg::PgConnection;

use config::ServerConfig;

lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = ServerConfig::load_from_env();
    pub static ref POOL: r2d2::Pool<ConnectionManager<PgConnection>> = make_pool();
}

pub fn server_config<'a>() -> &'a ServerConfig {
    &*SERVER_CONFIG
}

pub fn conn_pool<'a>() -> &'a r2d2::Pool<ConnectionManager<PgConnection>> {
    &(*POOL)
}

fn make_pool() -> r2d2::Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL")
                                                             .expect("DATABASE_URL must be set"));

    r2d2::Pool::new(config, manager).expect("Failed to create pool.")
}

#[test]
fn test_make_pool() {
    println!("测试获取数据库连接...");
    let pool = conn_pool();
    assert!(pool.get().is_ok());
}
