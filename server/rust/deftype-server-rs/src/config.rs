use std::fmt;
use std::env;

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl ServerConfig {
    pub fn new(host: &str, port: u16) -> Self {
        ServerConfig {
            host: host.to_owned(),
            port: port,
        }
    }

    pub fn load_from_env() -> ServerConfig {
        let d = Self::default();
        let default_port = d.port; // just copy. happy for capture of partially moved value: `d`
        let host = env::var("HOST").unwrap_or(d.host);
        let port = env::var("PORT")
                       .map(|p| p.parse::<u16>().unwrap_or(default_port))
                       .unwrap_or(default_port);
        Self::new(&host, port)
    }

    pub fn to_addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> ServerConfig {
        Self::new("localhost", 3000)
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ServerConfig(host:{} , port: {})", self.host, self.port)
    }
}
