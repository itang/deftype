use std::{fmt, env};
use std::str::FromStr;


static DEVELOPMENT: &'static str = "development";
static PRODUCTION: &'static str = "production";

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum RunMode {
    Development,
    Production,
}

impl RunMode {
    pub fn is_dev(&self) -> bool {
        *self == RunMode::Development
    }

    #[allow(dead_code)]
    pub fn is_prod(&self) -> bool {
        *self == RunMode::Production
    }

    pub fn to_str(&self) -> &str {
        match *self {
            RunMode::Development => DEVELOPMENT,
            RunMode::Production => PRODUCTION,
        }
    }
}

impl fmt::Display for RunMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

pub struct RunModeParseError;

impl FromStr for RunMode {
    type Err = RunModeParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "dev" | "development" => Ok(RunMode::Development),
            "prod" | "production" => Ok(RunMode::Production),
            _ => Err(RunModeParseError),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ServerConfig {
    pub run_mode: RunMode,
    pub host: String,
    pub port: u16,
    pub root_dir: String,
    pub auth_secret: String,
}

impl ServerConfig {
    fn new(run_mode: RunMode,
           host: String,
           port: u16,
           root_dir: String,
           auth_secret: String)
           -> Self {
        ServerConfig {
            run_mode: run_mode,
            host: host,
            port: port,
            root_dir: root_dir,
            auth_secret: auth_secret,
        }
    }

    pub fn load_from_env() -> Self {
        let d = Self::default();
        let default_port = d.port; // just copy. happy for capture of partially moved value: `d`
        let default_run_mode = d.run_mode;
        let host = env::var("HOST").unwrap_or(d.host);
        let root_dir = env::var("ROOT_DIR").unwrap_or(d.root_dir);
        let auth_secret = env::var("AUTH_SECRET").unwrap_or(d.auth_secret);
        let port = env::var("PORT")
                       .map(|p| p.parse::<u16>().unwrap_or(default_port))
                       .unwrap_or(default_port);
        let run_mode = env::var("RUN_MODE")
                           .map(|m| m.parse::<RunMode>().unwrap_or(default_run_mode))
                           .unwrap_or(default_run_mode);
        Self::new(run_mode, host, port, root_dir, auth_secret)
    }

    pub fn to_addr(&self) -> (&str, u16) {
        (&self.host, self.port)
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self::new(RunMode::Development,
                  "localhost".to_owned(),
                  3000,
                  "public/".to_owned(),
                  "uLkvkYvgiA01ozKoTvyyXL_YBZUxDQK0OGosXmdBg84=".to_owned())
    }
}

impl fmt::Display for ServerConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "ServerConfig(run_mode: {}, host:{} , port: {}, root_dir:{}, auth_secret:{})",
        self.run_mode,
               self.host,
               self.port,
               self.root_dir,
        self.auth_secret,
               )
    }
}
