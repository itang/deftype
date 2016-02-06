use std::fmt;
use std::env;

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub root_dir: String,
}

impl ServerConfig {
    fn new(host: String, port: u16, root_dir: String) -> Self {
        ServerConfig {
            host: host,
            port: port,
            root_dir: root_dir,
        }
    }

    pub fn load_from_env() -> ServerConfig {
        let d = Self::default();
        let default_port = d.port; // just copy. happy for capture of partially moved value: `d`
        let host = env::var("HOST").unwrap_or(d.host);
        let root_dir = env::var("ROOT_DIR").unwrap_or(d.root_dir);
        let port = env::var("PORT")
                       .map(|p| p.parse::<u16>().unwrap_or(default_port))
                       .unwrap_or(default_port);
        Self::new(host, port, root_dir)
    }

    pub fn to_addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        Self::new("localhost".to_owned(), 3000, "public/".to_owned())
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServerConfig(host:{} , port: {})", self.host, self.port)
    }
}
