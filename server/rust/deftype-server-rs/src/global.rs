use config::ServerConfig;

lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = ServerConfig::load_from_env();
}

pub fn server_config<'a>() -> &'a ServerConfig {
    &*SERVER_CONFIG
}
