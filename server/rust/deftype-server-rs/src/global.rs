use config::ServerConfig;

lazy_static! {
    pub static ref SERVER_CONFIG: ServerConfig = ServerConfig::load_from_env();
}
